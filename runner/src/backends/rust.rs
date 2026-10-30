//! Rust: a Cargo workspace with one library crate per exercise and
//! integration tests in `tests/spec.rs`.

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::Value;

use super::{case_name, def_of, doc, escape, field_values, header, hex_escape, render_args, step_label, timing, Backend};
use crate::naming;
use crate::spec::{CaseKind, DefKind, Function, Method, Outcome, Spec, Type, TypeDef};
use crate::toolchain::{Found, Invocation, Tool};

pub struct Rust;

static TOOL: Tool = Tool {
    label: "cargo",
    candidates: &["cargo", "~/.cargo/bin/cargo", "~/.cargo/bin/cargo.exe"],
    version_args: &["--version"],
    prefer: None,
};

const TASK: &str = "use std::thread;
use std::time::Duration;

/// Provided, don't change: a slow task. Sleeps for `ms` milliseconds, then returns `ms`.
pub fn task(ms: i64) -> i64 {
    thread::sleep(Duration::from_millis(ms.max(0) as u64));
    ms
}

";

fn str_lit(s: &str) -> String {
    format!("\"{}\"", escape(s, hex_escape, &[]))
}

impl Rust {
    /// Argument form: borrowed where the parameter is borrowed.
    fn arg_literal(&self, spec: &Spec, value: &Value, ty: &Type) -> String {
        match (ty, value) {
            (Type::Str, Value::String(s)) => str_lit(s),
            (Type::List(t), Value::Array(items)) => {
                let parts: Vec<String> = items.iter().map(|v| self.literal(spec, v, t)).collect();
                format!("&[{}]", parts.join(", "))
            }
            (Type::Map(_), _) => format!("&{}", self.literal(spec, value, ty)),
            (Type::Named(n), _) => format!("&{}", self.struct_literal(spec, def_of(spec, n, value), value)),
            _ => self.literal(spec, value, ty),
        }
    }

    fn struct_literal(&self, spec: &Spec, def: &TypeDef, value: &Value) -> String {
        let fields: Vec<String> = field_values(def, value)
            .into_iter()
            .map(|(f, v)| format!("{}: {}", naming::snake(&f.name), self.literal(spec, v, &f.ty)))
            .collect();
        format!("{} {{ {} }}", naming::pascal(&def.name), fields.join(", "))
    }

    fn crate_name(spec: &Spec) -> String {
        naming::snake(&spec.slug)
    }

    fn fn_signature(&self, spec: &Spec, name: &str, params: &[crate::spec::Param], returns: &Type, fallible: bool, receiver: bool) -> String {
        let mut ps: Vec<String> = Vec::new();
        if receiver {
            ps.push("&self".into());
        }
        ps.extend(params.iter().map(|p| format!("{}: {}", p.name, self.type_name(spec, &p.ty, true))));
        let ret = self.type_name(spec, returns, false);
        let ret = if fallible { format!("Result<{ret}, String>") } else { ret };
        format!("fn {name}({}) -> {ret}", ps.join(", "))
    }

    fn method_sig(&self, spec: &Spec, m: &Method) -> String {
        self.fn_signature(spec, &naming::snake(&m.name), &m.params, &m.returns, false, true)
    }

    fn type_defs(&self, spec: &Spec) -> String {
        let mut s = String::new();
        for d in &spec.types {
            let name = naming::pascal(&d.name);
            match d.kind {
                DefKind::Interface => {
                    s.push_str(&format!("pub trait {name} {{\n"));
                    for m in &d.methods {
                        s.push_str(&format!("    {};\n", self.method_sig(spec, m)));
                    }
                    s.push_str("}\n\n");
                }
                DefKind::Record | DefKind::Class => {
                    let derive = if d.kind == DefKind::Record { "Debug, Clone, PartialEq, Eq" } else { "Debug, Clone" };
                    s.push_str(&format!("#[derive({derive})]\npub struct {name} {{\n"));
                    for f in &d.fields {
                        s.push_str(&format!("    pub {}: {},\n", naming::snake(&f.name), self.type_name(spec, &f.ty, false)));
                    }
                    s.push_str("}\n\n");
                    // One impl block per interface, plus one for the class's own methods.
                    let mut groups: Vec<(Option<&str>, Vec<&Method>)> = Vec::new();
                    for m in &d.methods {
                        let iface = d.implements.iter().find(|i| {
                            spec.def(i).is_some_and(|idef| idef.methods.iter().any(|im| im.name == m.name))
                        });
                        let key = iface.map(String::as_str);
                        match groups.iter_mut().find(|(k, _)| *k == key) {
                            Some((_, ms)) => ms.push(m),
                            None => groups.push((key, vec![m])),
                        }
                    }
                    for (iface, methods) in groups {
                        match iface {
                            Some(i) => s.push_str(&format!("impl {} for {name} {{\n", naming::pascal(i))),
                            None => s.push_str(&format!("impl {name} {{\n")),
                        }
                        let vis = if iface.is_some() { "" } else { "pub " };
                        for (k, m) in methods.iter().enumerate() {
                            if k > 0 {
                                s.push('\n');
                            }
                            s.push_str(&format!(
                                "    {vis}{} {{\n        todo!(\"implement {name}::{}\")\n    }}\n",
                                self.method_sig(spec, m),
                                naming::snake(&m.name)
                            ));
                        }
                        s.push_str("}\n\n");
                    }
                }
            }
        }
        s
    }

    fn fn_stub(&self, spec: &Spec, f: &Function) -> String {
        let mut s = doc("/// ", &f.summary);
        if f.fallible {
            s.push_str("///\n/// Return `Err` with a message for invalid input; don't panic.\n");
        }
        let name = self.function_name(f);
        // Show the arguments in the panic message; trait objects aren't Debug, so
        // those are only touched (to avoid unused-variable warnings).
        let debuggable = |t: &Type| {
            !t.any_named(&spec.types, &mut |n| spec.def(n).is_some_and(|d| d.kind != DefKind::Record))
        };
        let shown: Vec<String> =
            f.params.iter().map(|p| if debuggable(&p.ty) { format!("{{{}:?}}", p.name) } else { "..".into() }).collect();
        let touched: String = f
            .params
            .iter()
            .filter(|p| !debuggable(&p.ty))
            .map(|p| format!("let _ = &{};\n    ", p.name))
            .collect();
        let todo = format!("{touched}todo!(\"implement {name}({})\")", shown.join(", "));
        let body = match &f.returns {
            // `impl FnMut` needs a closure to infer from; this one panics when called.
            Type::Fn(ps, _) => {
                let args: Vec<String> =
                    ps.iter().enumerate().map(|(i, t)| format!("_a{i}: {}", self.type_name(spec, t, false))).collect();
                format!("move |{}| {todo}", args.join(", "))
            }
            _ => todo,
        };
        s.push_str(&format!("{} {{\n    {body}\n}}\n", self.signature(spec, f)));
        s
    }

    fn call(&self, spec: &Spec, f: &Function, input: &[Value]) -> String {
        let path = match &f.module {
            Some(m) => format!("{}::{}", naming::snake(m), self.function_name(f)),
            None => self.function_name(f),
        };
        format!("{path}({})", render_args(&f.params, input, |v, t| self.arg_literal(spec, v, t)))
    }
}

impl Backend for Rust {
    fn id(&self) -> &'static str {
        "rust"
    }
    fn name(&self) -> &'static str {
        "Rust"
    }
    fn fence(&self) -> &'static str {
        "rust"
    }
    fn tool(&self) -> &'static Tool {
        &TOOL
    }

    fn function_name(&self, f: &Function) -> String {
        naming::snake(&f.name)
    }

    fn type_name(&self, spec: &Spec, ty: &Type, param: bool) -> String {
        match ty {
            Type::Int => "i64".into(),
            Type::Bool => "bool".into(),
            Type::Str if param => "&str".into(),
            Type::Str => "String".into(),
            Type::List(t) if param => format!("&[{}]", self.type_name(spec, t, false)),
            Type::List(t) => format!("Vec<{}>", self.type_name(spec, t, false)),
            Type::Map(t) if param => format!("&HashMap<String, {}>", self.type_name(spec, t, false)),
            Type::Map(t) => format!("HashMap<String, {}>", self.type_name(spec, t, false)),
            Type::Named(n) => {
                let interface = spec.def(n).is_some_and(|d| d.kind == DefKind::Interface);
                match (interface, param) {
                    (true, true) => format!("&dyn {}", naming::pascal(n)),
                    (true, false) => format!("Box<dyn {}>", naming::pascal(n)),
                    (false, true) => format!("&{}", naming::pascal(n)),
                    (false, false) => naming::pascal(n),
                }
            }
            Type::Fn(ps, r) => {
                let ps: Vec<String> = ps.iter().map(|t| self.type_name(spec, t, false)).collect();
                format!("impl FnMut({}) -> {}", ps.join(", "), self.type_name(spec, r, false))
            }
        }
    }

    /// Owned literal. Empty collections rely on the surrounding type annotation.
    fn literal(&self, spec: &Spec, value: &Value, ty: &Type) -> String {
        match (ty, value) {
            (Type::Int, Value::Number(n)) => n.to_string(),
            (Type::Bool, Value::Bool(b)) => b.to_string(),
            (Type::Str, Value::String(s)) => format!("String::from({})", str_lit(s)),
            (Type::List(t), Value::Array(items)) => {
                let parts: Vec<String> = items.iter().map(|v| self.literal(spec, v, t)).collect();
                format!("vec![{}]", parts.join(", "))
            }
            (Type::Map(_), Value::Object(m)) if m.is_empty() => "HashMap::new()".into(),
            (Type::Map(t), Value::Object(m)) => {
                let parts: Vec<String> = m
                    .iter()
                    .map(|(k, v)| format!("(String::from({}), {})", str_lit(k), self.literal(spec, v, t)))
                    .collect();
                format!("HashMap::from([{}])", parts.join(", "))
            }
            (Type::Named(n), _) => {
                let def = def_of(spec, n, value);
                let lit = self.struct_literal(spec, def, value);
                if spec.def(n).is_some_and(|d| d.kind == DefKind::Interface) {
                    format!("Box::new({lit}) as Box<dyn {}>", naming::pascal(n))
                } else {
                    lit
                }
            }
            _ => unreachable!("spec values are validated against their types"),
        }
    }

    fn signature(&self, spec: &Spec, f: &Function) -> String {
        format!("pub {}", self.fn_signature(spec, &self.function_name(f), &f.params, &f.returns, f.fallible, false))
    }

    fn stub_path(&self, spec: &Spec) -> PathBuf {
        PathBuf::from(&spec.slug).join("src").join("lib.rs")
    }

    fn module_path(&self, spec: &Spec, module: &str) -> PathBuf {
        PathBuf::from(&spec.slug).join("src").join(format!("{}.rs", naming::snake(module)))
    }

    fn stubs(&self, spec: &Spec) -> Vec<(PathBuf, String)> {
        let mut main = String::new();
        for m in spec.modules() {
            main.push_str(&format!("pub mod {};\n", naming::snake(m)));
        }
        if !spec.modules().is_empty() {
            main.push('\n');
        }
        if spec.uses_map() {
            main.push_str("use std::collections::HashMap;\n\n");
        }
        if spec.concurrency {
            main.push_str(TASK);
        }
        main.push_str(&self.type_defs(spec));
        main.push_str(&self.fn_stub(spec, &spec.main));
        let mut out = vec![(self.stub_path(spec), main)];

        for m in spec.modules() {
            let mut s = String::new();
            for (i, f) in spec.helpers.iter().filter(|h| h.module.as_deref() == Some(m)).enumerate() {
                if i > 0 {
                    s.push('\n');
                }
                s.push_str(&self.fn_stub(spec, f));
            }
            out.push((self.module_path(spec, m), s));
        }
        out
    }

    fn tests(&self, spec: &Spec) -> Vec<(PathBuf, String)> {
        let mut s = String::from("#![allow(unused_imports, unused_mut)]\n");
        s.push_str(&header("//", spec, "src/"));
        s.push('\n');
        if spec.uses_map() {
            s.push_str("use std::collections::HashMap;\n");
        }
        s.push_str(&format!("use {}::*;\n", Self::crate_name(spec)));

        for (f, case) in spec.cases() {
            s.push_str(&format!("\n#[test]\nfn {}() {{\n", case_name(case)));
            match &case.kind {
                CaseKind::Call { input, outcome } => {
                    let call = self.call(spec, f, input);
                    match outcome {
                        Outcome::Value(v) => {
                            s.push_str(&format!(
                                "    let expected: {} = {};\n",
                                self.type_name(spec, &f.returns, false),
                                self.literal(spec, v, &f.returns)
                            ));
                            if spec.concurrency {
                                let t = timing(input);
                                s.push_str(&format!(
                                    "    let start = std::time::Instant::now();\n    let got = {call};\n    let elapsed = start.elapsed().as_millis() as i64;\n    assert_eq!(got, expected);\n    assert!(elapsed >= {}, \"finished in {{elapsed}} ms, but the slowest task alone takes {} ms: call task() for every delay\");\n    assert!(elapsed < {}, \"took {{elapsed}} ms: run the tasks at the same time so the whole call takes about {} ms, not {} ms\");\n",
                                    t.min_ms, t.max, t.limit_ms, t.max, t.sum
                                ));
                            } else if f.fallible {
                                s.push_str(&format!("    assert_eq!({call}, Ok(expected));\n"));
                            } else {
                                s.push_str(&format!("    assert_eq!({call}, expected);\n"));
                            }
                        }
                        Outcome::Error => s.push_str(&format!(
                            "    let result = {call};\n    assert!(result.is_err(), \"expected an error, got {{result:?}}\");\n"
                        )),
                    }
                }
                CaseKind::Scenario { create, steps } => {
                    for (i, args) in create.iter().enumerate() {
                        s.push_str(&format!("    let mut v{i} = {};\n", self.call(spec, f, args)));
                    }
                    let Type::Fn(ps, ret) = &f.returns else { unreachable!() };
                    for (n, step) in steps.iter().enumerate() {
                        let args: Vec<String> =
                            step.input.iter().zip(ps).map(|(v, t)| self.literal(spec, v, t)).collect();
                        s.push_str(&format!(
                            "    let expected: {} = {};\n    assert_eq!(v{}({}), expected, \"{}\");\n",
                            self.type_name(spec, ret, false),
                            self.literal(spec, &step.expected, ret),
                            step.on,
                            args.join(", "),
                            step_label(n + 1, step.on, create.len())
                        ));
                    }
                }
            }
            s.push_str("}\n");
        }
        vec![(PathBuf::from(&spec.slug).join("tests").join("spec.rs"), s)]
    }

    fn scaffold(&self, spec: &Spec) -> Vec<(PathBuf, String)> {
        let manifest = format!(
            "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n",
            spec.slug
        );
        vec![(PathBuf::from(&spec.slug).join("Cargo.toml"), manifest)]
    }

    fn shared(&self, specs: &[Spec]) -> Vec<(PathBuf, String)> {
        let members: Vec<String> = specs.iter().map(|s| format!("    \"{}\",", s.slug)).collect();
        let manifest = format!(
            "# Generated by `learn generate`; overwritten when the course changes.\n\
             [workspace]\nresolver = \"2\"\nmembers = [\n{}\n]\n",
            members.join("\n")
        );
        vec![(PathBuf::from("Cargo.toml"), manifest)]
    }

    fn command(&self, spec: &Spec, tool: &Found, track_dir: &Path) -> Invocation {
        Invocation::new(&tool.path, track_dir)
            .args(["test", "-p", spec.slug.as_str(), "--test", "spec", "--color", "never"])
            .env("CARGO_TERM_COLOR", "never")
            .timeout(Duration::from_secs(600))
    }

    /// `---- name stdout ----`, then `thread '…' panicked at file:line:`, then the message.
    fn failure_reason(&self, output: &str, test_name: &str) -> Option<String> {
        let header = format!("---- {test_name} stdout ----");
        let block = super::lines_after(output, |l| l.trim() == header, |l| l.starts_with("---- ") || l.trim() == "failures:", 12)?;
        let msg: Vec<&str> = block
            .lines()
            .skip_while(|l| !l.contains("panicked at"))
            .skip(1)
            .take_while(|l| !l.starts_with("note:"))
            .collect();
        (!msg.is_empty()).then(|| msg.join("\n"))
    }

    fn parse_results(&self, output: &str) -> Vec<(usize, bool)> {
        output
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                let rest = line.strip_prefix("test ")?;
                let passed = if rest.ends_with("... ok") {
                    true
                } else if rest.ends_with("... FAILED") {
                    false
                } else {
                    return None;
                };
                Some((naming::find_case_index(rest)?, passed))
            })
            .collect()
    }
}
