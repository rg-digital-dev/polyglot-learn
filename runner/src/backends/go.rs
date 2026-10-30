//! Go: one module with a package per exercise, tested with `go test ./<slug>/`.
//! Helper modules become subpackages (`learn/<slug>/<module>`).

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::Value;

use super::{case_name, def_of, doc, escape, field_values, header, hex_escape, kept_inputs, render_args, render_kept_args, step_label, stub_message, timing, Backend};
use crate::naming;
use crate::spec::{CaseKind, DefKind, Function, Method, Outcome, Param, Spec, Type};
use crate::toolchain::{Found, Invocation, Tool};

pub struct Go;

static TOOL: Tool = Tool {
    label: "go",
    candidates: &[
        "go",
        "/usr/local/go/bin/go",
        "/usr/lib/go/bin/go",
        "/opt/homebrew/bin/go",
        "C:/Program Files/Go/bin/go.exe",
        "~/go/bin/go",
    ],
    version_args: &["version"],
    prefer: None,
};

const HELPERS: &str = r#"
// specEqual compares results structurally. Unlike reflect.DeepEqual it treats
// a nil slice or map as equal to an empty one, so `return nil` is fine.
func specEqual(a, b any) bool {
	return valuesEqual(reflect.ValueOf(a), reflect.ValueOf(b))
}

func valuesEqual(a, b reflect.Value) bool {
	if !a.IsValid() || !b.IsValid() {
		return a.IsValid() == b.IsValid()
	}
	if a.Kind() != b.Kind() {
		return false
	}
	switch a.Kind() {
	case reflect.Slice:
		if a.Len() != b.Len() {
			return false
		}
		for i := 0; i < a.Len(); i++ {
			if !valuesEqual(a.Index(i), b.Index(i)) {
				return false
			}
		}
		return true
	case reflect.Map:
		if a.Len() != b.Len() {
			return false
		}
		for _, k := range a.MapKeys() {
			bv := b.MapIndex(k)
			if !bv.IsValid() || !valuesEqual(a.MapIndex(k), bv) {
				return false
			}
		}
		return true
	case reflect.Struct:
		if a.Type() != b.Type() {
			return false
		}
		for i := 0; i < a.NumField(); i++ {
			if !valuesEqual(a.Field(i), b.Field(i)) {
				return false
			}
		}
		return true
	default:
		return reflect.DeepEqual(a.Interface(), b.Interface())
	}
}

// failOnPanic turns a panic (like the stub's) into a normal test failure, so
// one panicking case doesn't stop the others from running.
func failOnPanic(t *testing.T) {
	if r := recover(); r != nil {
		t.Fatalf("panicked: %v", r)
	}
}
"#;

const TASK: &str = "// task is provided; don't change it. It sleeps for ms milliseconds, then returns ms.
func task(ms int) int {
\ttime.Sleep(time.Duration(ms) * time.Millisecond)
\treturn ms
}

";

fn str_lit(s: &str) -> String {
    format!("\"{}\"", escape(s, hex_escape, &[]))
}

impl Go {
    fn package(spec: &Spec) -> String {
        naming::flat(&spec.slug)
    }

    fn params(&self, spec: &Spec, params: &[Param]) -> String {
        params
            .iter()
            .map(|p| format!("{} {}", naming::camel(&p.name), self.type_name(spec, &p.ty, true)))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn method_sig(&self, spec: &Spec, m: &Method) -> String {
        format!("{}({}) {}", naming::pascal(&m.name), self.params(spec, &m.params), self.type_name(spec, &m.returns, false))
    }

    fn type_defs(&self, spec: &Spec) -> String {
        let mut s = String::new();
        for d in &spec.types {
            let name = naming::pascal(&d.name);
            match d.kind {
                DefKind::Interface => {
                    s.push_str(&format!("type {name} interface {{\n"));
                    for m in &d.methods {
                        s.push_str(&format!("\t{}\n", self.method_sig(spec, m)));
                    }
                    s.push_str("}\n\n");
                }
                DefKind::Record | DefKind::Class => {
                    s.push_str(&format!("type {name} struct {{\n"));
                    for f in &d.fields {
                        s.push_str(&format!("\t{} {}\n", naming::pascal(&f.name), self.type_name(spec, &f.ty, false)));
                    }
                    s.push_str("}\n\n");
                    let recv = name.chars().next().unwrap_or('x').to_ascii_lowercase();
                    for m in &d.methods {
                        s.push_str(&format!(
                            "func ({recv} {name}) {} {{\n\tpanic(\"{}\")\n}}\n\n",
                            self.method_sig(spec, m),
                            stub_message(&format!("{name}.{}", naming::pascal(&m.name)))
                        ));
                    }
                }
            }
        }
        s
    }

    fn fn_stub(&self, spec: &Spec, f: &Function) -> String {
        let name = self.function_name(f);
        let mut s = doc("// ", &format!("{name}: {}", f.summary));
        if f.fallible {
            s.push_str("//\n// Return a non-nil error for invalid input; don't panic.\n");
        }
        s.push_str(&format!("{} {{\n\tpanic(\"{}\")\n}}\n", self.signature(spec, f), stub_message(&name)));
        s
    }

    fn call(&self, spec: &Spec, f: &Function, input: &[Value]) -> String {
        let path = match &f.module {
            Some(m) => format!("{}.{}", naming::flat(m), self.function_name(f)),
            None => self.function_name(f),
        };
        format!("{path}({})", render_args(&f.params, input, |v, t| self.literal(spec, v, t)))
    }
}

impl Backend for Go {
    fn id(&self) -> &'static str {
        "go"
    }
    fn name(&self) -> &'static str {
        "Go"
    }
    fn fence(&self) -> &'static str {
        "go"
    }
    fn tool(&self) -> &'static Tool {
        &TOOL
    }

    fn function_name(&self, f: &Function) -> String {
        naming::pascal(&f.name)
    }

    fn type_name(&self, spec: &Spec, ty: &Type, _param: bool) -> String {
        match ty {
            Type::Int => "int".into(),
            Type::Bool => "bool".into(),
            Type::Str => "string".into(),
            Type::List(t) => format!("[]{}", self.type_name(spec, t, false)),
            Type::Map(t) => format!("map[string]{}", self.type_name(spec, t, false)),
            Type::Named(n) => naming::pascal(n),
            Type::Fn(ps, r) => {
                let ps: Vec<String> = ps.iter().map(|t| self.type_name(spec, t, false)).collect();
                format!("func({}) {}", ps.join(", "), self.type_name(spec, r, false))
            }
        }
    }

    fn literal(&self, spec: &Spec, value: &Value, ty: &Type) -> String {
        match (ty, value) {
            (Type::Int, Value::Number(n)) => n.to_string(),
            (Type::Bool, Value::Bool(b)) => b.to_string(),
            (Type::Str, Value::String(s)) => str_lit(s),
            (Type::List(t), Value::Array(items)) => {
                let parts: Vec<String> = items.iter().map(|v| self.literal(spec, v, t)).collect();
                format!("{}{{{}}}", self.type_name(spec, ty, false), parts.join(", "))
            }
            (Type::Map(t), Value::Object(m)) => {
                let parts: Vec<String> =
                    m.iter().map(|(k, v)| format!("{}: {}", str_lit(k), self.literal(spec, v, t))).collect();
                format!("{}{{{}}}", self.type_name(spec, ty, false), parts.join(", "))
            }
            (Type::Named(n), _) => {
                let def = def_of(spec, n, value);
                let fields: Vec<String> = field_values(def, value)
                    .into_iter()
                    .map(|(f, v)| format!("{}: {}", naming::pascal(&f.name), self.literal(spec, v, &f.ty)))
                    .collect();
                format!("{}{{{}}}", naming::pascal(&def.name), fields.join(", "))
            }
            _ => unreachable!("spec values are validated against their types"),
        }
    }

    fn signature(&self, spec: &Spec, f: &Function) -> String {
        let ret = self.type_name(spec, &f.returns, false);
        let ret = if f.fallible { format!("({ret}, error)") } else { ret };
        format!("func {}({}) {ret}", self.function_name(f), self.params(spec, &f.params))
    }

    fn stub_path(&self, spec: &Spec) -> PathBuf {
        PathBuf::from(&spec.slug).join(format!("{}.go", Self::package(spec)))
    }

    fn module_path(&self, spec: &Spec, module: &str) -> PathBuf {
        let pkg = naming::flat(module);
        PathBuf::from(&spec.slug).join(&pkg).join(format!("{pkg}.go"))
    }

    fn stubs(&self, spec: &Spec) -> Vec<(PathBuf, String)> {
        let mut main = format!("package {}\n\n", Self::package(spec));
        if spec.concurrency {
            main.push_str("import \"time\"\n\n");
            main.push_str(TASK);
        }
        main.push_str(&self.type_defs(spec));
        main.push_str(&self.fn_stub(spec, &spec.main));
        let mut out = vec![(self.stub_path(spec), main)];
        for m in spec.modules() {
            let mut s = format!("package {}\n", naming::flat(m));
            for f in spec.helpers.iter().filter(|h| h.module.as_deref() == Some(m)) {
                s.push('\n');
                s.push_str(&self.fn_stub(spec, f));
            }
            out.push((self.module_path(spec, m), s));
        }
        out
    }

    fn tests(&self, spec: &Spec) -> Vec<(PathBuf, String)> {
        let pkg = Self::package(spec);
        let mut s = header("//", spec, &format!("{pkg}.go"));
        let mut imports = vec!["\"reflect\"".to_string(), "\"testing\"".to_string()];
        if spec.concurrency {
            imports.push("\"time\"".into());
        }
        for m in spec.modules() {
            imports.push(format!("\"learn/{}/{}\"", spec.slug, naming::flat(m)));
        }
        s.push_str(&format!("\npackage {pkg}\n\nimport (\n"));
        for i in imports {
            s.push_str(&format!("\t{i}\n"));
        }
        s.push_str(")\n");

        for f in spec.functions() {
            s.push_str(&format!("\nfunc Test{}(t *testing.T) {{\n", self.function_name(f)));
            for case in &f.cases {
                s.push_str(&format!("\tt.Run(\"{}\", func(t *testing.T) {{\n\t\tdefer failOnPanic(t)\n", case_name(case)));
                match &case.kind {
                    CaseKind::Call { input, outcome } => {
                        let call = self.call(spec, f, input);
                        match (outcome, f.fallible) {
                            (Outcome::Value(v), false) => {
                                s.push_str(&format!("\t\twant := {}\n", self.literal(spec, v, &f.returns)));
                                if spec.concurrency {
                                    let tm = timing(input);
                                    s.push_str(&format!(
                                        "\t\tstart := time.Now()\n\t\tgot := {call}\n\t\telapsed := time.Since(start).Milliseconds()\n\t\tif !specEqual(got, want) {{\n\t\t\tt.Fatalf(\"got %#v, want %#v\", got, want)\n\t\t}}\n\t\tif elapsed < {} {{\n\t\t\tt.Fatalf(\"finished in %d ms, but the slowest task alone takes {} ms: call task for every delay\", elapsed)\n\t\t}}\n\t\tif elapsed >= {} {{\n\t\t\tt.Fatalf(\"took %d ms: run the tasks at the same time so the call takes about {} ms, not {} ms\", elapsed)\n\t\t}}\n",
                                        tm.min_ms, tm.max, tm.limit_ms, tm.max, tm.sum
                                    ));
                                } else if kept_inputs(spec, f).iter().any(Option::is_some) {
                                    // Slices and maps share memory with the caller: check
                                    // the function left them alone.
                                    let kept = kept_inputs(spec, f);
                                    let (args, vars) =
                                        render_kept_args(f, input, &kept, |v, t| self.literal(spec, v, t), naming::camel);
                                    for (var, lit, _) in &vars {
                                        s.push_str(&format!("\t\t{var} := {lit}\n"));
                                    }
                                    s.push_str(&format!(
                                        "\t\tgot := {}({args})\n\t\tif !specEqual(got, want) {{\n\t\t\tt.Fatalf(\"got %#v, want %#v\", got, want)\n\t\t}}\n",
                                        self.function_name(f)
                                    ));
                                    for (var, lit, kind) in &vars {
                                        s.push_str(&format!(
                                            "\t\tif !reflect.DeepEqual({var}, {lit}) {{\n\t\t\tt.Fatalf(\"the input {kind} was modified: it is now %#v\", {var})\n\t\t}}\n"
                                        ));
                                    }
                                } else {
                                    s.push_str(&format!(
                                        "\t\tgot := {call}\n\t\tif !specEqual(got, want) {{\n\t\t\tt.Fatalf(\"got %#v, want %#v\", got, want)\n\t\t}}\n"
                                    ));
                                }
                            }
                            (Outcome::Value(v), true) => s.push_str(&format!(
                                "\t\twant := {}\n\t\tgot, err := {call}\n\t\tif err != nil {{\n\t\t\tt.Fatalf(\"unexpected error: %v\", err)\n\t\t}}\n\t\tif !specEqual(got, want) {{\n\t\t\tt.Fatalf(\"got %#v, want %#v\", got, want)\n\t\t}}\n",
                                self.literal(spec, v, &f.returns)
                            )),
                            (Outcome::Error, _) => s.push_str(&format!(
                                "\t\tgot, err := {call}\n\t\tif err == nil {{\n\t\t\tt.Fatalf(\"expected an error, got %#v\", got)\n\t\t}}\n"
                            )),
                        }
                    }
                    CaseKind::Scenario { create, steps } => {
                        for (i, args) in create.iter().enumerate() {
                            s.push_str(&format!("\t\tv{i} := {}\n\t\t_ = v{i}\n", self.call(spec, f, args)));
                        }
                        let Type::Fn(ps, ret) = &f.returns else { unreachable!() };
                        for (n, step) in steps.iter().enumerate() {
                            let args: Vec<String> =
                                step.input.iter().zip(ps).map(|(v, t)| self.literal(spec, v, t)).collect();
                            s.push_str(&format!(
                                "\t\tif got, want := v{}({}), {}; !specEqual(got, want) {{\n\t\t\tt.Fatalf(\"{}: got %#v, want %#v\", got, want)\n\t\t}}\n",
                                step.on,
                                args.join(", "),
                                self.literal(spec, &step.expected, ret),
                                step_label(n + 1, step.on, create.len())
                            ));
                        }
                    }
                }
                s.push_str("\t})\n");
            }
            s.push_str("}\n");
        }
        s.push_str(HELPERS);
        vec![(PathBuf::from(&spec.slug).join(format!("{pkg}_test.go")), s)]
    }

    fn shared(&self, _specs: &[Spec]) -> Vec<(PathBuf, String)> {
        vec![(PathBuf::from("go.mod"), "module learn\n\ngo 1.22\n".to_string())]
    }

    fn command(&self, spec: &Spec, tool: &Found, track_dir: &Path) -> Invocation {
        Invocation::new(&tool.path, track_dir)
            .args(["test", "-v", "-count=1", &format!("./{}/", spec.slug)])
            .timeout(Duration::from_secs(300))
    }

    /// Lines like `    x_test.go:42: got …` between `=== RUN …/name` and its `--- FAIL`.
    fn failure_reason(&self, output: &str, test_name: &str) -> Option<String> {
        let suffix = format!("/{test_name}");
        let block = super::lines_after(
            output,
            |l| l.trim().starts_with("=== RUN") && l.trim().ends_with(&suffix),
            |l| l.trim().starts_with("--- ") || l.trim().starts_with("=== RUN"),
            8,
        )?;
        let msg: Vec<String> = block
            .lines()
            .map(|l| match l.find(".go:").and_then(|i| l[i..].find(": ").map(|j| i + j + 2)) {
                Some(start) => l[start..].to_string(),
                None => l.trim().to_string(),
            })
            .collect();
        Some(msg.join("\n"))
    }

    fn parse_results(&self, output: &str) -> Vec<(usize, bool)> {
        output
            .lines()
            .filter_map(|line| {
                let t = line.trim();
                let passed = if t.starts_with("--- PASS:") {
                    true
                } else if t.starts_with("--- FAIL:") {
                    false
                } else {
                    return None;
                };
                Some((naming::find_case_index(t)?, passed))
            })
            .collect()
    }

    fn check_tool(&self, tool: &Found) -> Option<String> {
        let (major, minor, _) = crate::toolchain::parse_version(&tool.version)?;
        (major == 1 && minor < 22).then(|| format!("Go 1.22 or newer is needed (found {})", tool.version))
    }
}
