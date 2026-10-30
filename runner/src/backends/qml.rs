//! QML (Qt 6): the solution is a `QtObject` component with functions; the
//! tests are a `TestCase` run by `qmltestrunner -platform offscreen`.
//! "Classes" are factory functions returning JS objects; helper modules are
//! ECMAScript modules (`.mjs`); concurrency uses generators on a virtual clock.

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::Value;

use super::{case_name, def_of, doc, field_values, header, json, kept_inputs, render_args, render_kept_args, step_label, stub_message, timing, Backend};
use crate::naming;
use crate::spec::{CaseKind, DefKind, Function, Outcome, Param, Spec, Type};
use crate::toolchain::{Found, Invocation, Tool};

pub struct Qml;

static TOOL: Tool = Tool {
    label: "qmltestrunner",
    candidates: &[
        "qmltestrunner",
        "qmltestrunner6",
        "qmltestrunner-qt6",
        "/usr/lib/qt6/bin/qmltestrunner",
        "/usr/lib64/qt6/bin/qmltestrunner",
        "/usr/lib/x86_64-linux-gnu/qt6/bin/qmltestrunner",
        "/usr/lib/aarch64-linux-gnu/qt6/bin/qmltestrunner",
        "/opt/homebrew/opt/qt/bin/qmltestrunner",
        "/usr/local/opt/qt/bin/qmltestrunner",
        "~/Qt/6.*/macos/bin/qmltestrunner",
        "~/Qt/6.*/gcc_64/bin/qmltestrunner",
        "C:/Qt/6.*/msvc*_64/bin/qmltestrunner.exe",
        "C:/Qt/6.*/mingw*_64/bin/qmltestrunner.exe",
        "C:/Qt/6.*/llvm-mingw*_64/bin/qmltestrunner.exe",
    ],
    version_args: &[],
    prefer: None,
};

const CLOCK: &str = r#"    // Provided, don't change: a virtual clock. Call tick() once per
    // scheduling round; `now` counts the rounds.
    property int now: 0

    function tick() {
        now++
    }

    // Provided, don't change: a slow task as a generator. Advance it with
    // .next(); it reports { done: true, value: ms } once `ms` ticks have passed.
    function task(ms: int): var {
        return (function* () {
            const doneAt = root.now + ms
            while (root.now < doneAt)
                yield
            return ms
        })()
    }

"#;

impl Qml {
    fn component(spec: &Spec) -> String {
        naming::pascal(&spec.slug)
    }

    fn params(&self, spec: &Spec, params: &[Param]) -> String {
        params
            .iter()
            .map(|p| format!("{}: {}", naming::camel(&p.name), self.type_name(spec, &p.ty, true)))
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Records are documented; classes become factory functions returning objects.
    fn type_defs(&self, spec: &Spec) -> String {
        let mut s = String::new();
        for d in &spec.types {
            let name = naming::pascal(&d.name);
            match d.kind {
                DefKind::Interface => {
                    let ms: Vec<String> = d.methods.iter().map(|m| format!("{}()", naming::camel(&m.name))).collect();
                    s.push_str(&format!("    // {name}: any object with the methods {}.\n\n", ms.join(" and ")));
                }
                DefKind::Record => {
                    let fs: Vec<String> = d
                        .fields
                        .iter()
                        .map(|f| format!("{}: {}", naming::camel(&f.name), self.type_name(spec, &f.ty, false)))
                        .collect();
                    s.push_str(&format!("    // {name}: a plain object {{ {} }}\n\n", fs.join(", ")));
                }
                DefKind::Class => {
                    let factory = naming::camel(&d.name);
                    let iface = d.implements.first().map(|i| format!(", a {}", naming::pascal(i))).unwrap_or_default();
                    s.push_str(&format!("    // {name}{iface}: a factory function returning an object with methods.\n"));
                    s.push_str(&format!("    function {factory}({}): var {{\n        return {{\n", self.params(spec, &d.fields)));
                    for f in &d.fields {
                        let n = naming::camel(&f.name);
                        s.push_str(&format!("            {n}: {n},\n"));
                    }
                    for (i, m) in d.methods.iter().enumerate() {
                        let ps: Vec<String> = m.params.iter().map(|p| naming::camel(&p.name)).collect();
                        let comma = if i + 1 < d.methods.len() { "," } else { "" };
                        s.push_str(&format!(
                            "            {}({}) {{\n                throw new Error(\"{}\")\n            }}{comma}\n",
                            naming::camel(&m.name),
                            ps.join(", "),
                            stub_message(&format!("{factory}.{}", naming::camel(&m.name)))
                        ));
                    }
                    s.push_str("        }\n    }\n\n");
                }
            }
        }
        s
    }

    fn call(&self, spec: &Spec, f: &Function, input: &[Value]) -> String {
        let path = match &f.module {
            Some(m) => format!("{}.{}", naming::pascal(m), self.function_name(f)),
            None => format!("subject.{}", self.function_name(f)),
        };
        format!("{path}({})", render_args(&f.params, input, |v, t| self.literal(spec, v, t)))
    }
}

impl Backend for Qml {
    fn id(&self) -> &'static str {
        "qml"
    }
    fn name(&self) -> &'static str {
        "QML"
    }
    fn fence(&self) -> &'static str {
        "qml"
    }
    fn tool(&self) -> &'static Tool {
        &TOOL
    }

    fn function_name(&self, f: &Function) -> String {
        naming::camel(&f.name)
    }

    fn type_name(&self, _spec: &Spec, ty: &Type, _param: bool) -> String {
        match ty {
            Type::Int => "int".into(),
            Type::Bool => "bool".into(),
            Type::Str => "string".into(),
            Type::List(_) | Type::Map(_) | Type::Named(_) | Type::Fn(..) => "var".into(),
        }
    }

    fn literal(&self, spec: &Spec, value: &Value, ty: &Type) -> String {
        match (ty, value) {
            (Type::List(t), Value::Array(items)) => {
                let parts: Vec<String> = items.iter().map(|v| self.literal(spec, v, t)).collect();
                format!("[{}]", parts.join(", "))
            }
            (Type::Map(t), Value::Object(m)) => {
                let parts: Vec<String> =
                    m.iter().map(|(k, v)| format!("{}: {}", json(&Value::String(k.clone())), self.literal(spec, v, t))).collect();
                format!("({{{}}})", parts.join(", "))
            }
            (Type::Named(n), _) => {
                let def = def_of(spec, n, value);
                let fields = field_values(def, value);
                if def.kind == DefKind::Class {
                    let args: Vec<String> = fields.into_iter().map(|(f, v)| self.literal(spec, v, &f.ty)).collect();
                    format!("subject.{}({})", naming::camel(&def.name), args.join(", "))
                } else {
                    let parts: Vec<String> = fields
                        .into_iter()
                        .map(|(f, v)| format!("{}: {}", naming::camel(&f.name), self.literal(spec, v, &f.ty)))
                        .collect();
                    format!("({{ {} }})", parts.join(", "))
                }
            }
            _ => json(value),
        }
    }

    fn signature(&self, spec: &Spec, f: &Function) -> String {
        if f.module.is_some() {
            let ps: Vec<String> = f.params.iter().map(|p| naming::camel(&p.name)).collect();
            return format!("export function {}({})", self.function_name(f), ps.join(", "));
        }
        format!(
            "function {}({}): {}",
            self.function_name(f),
            self.params(spec, &f.params),
            self.type_name(spec, &f.returns, false)
        )
    }

    fn stub_path(&self, spec: &Spec) -> PathBuf {
        PathBuf::from(&spec.slug).join(format!("{}.qml", Self::component(spec)))
    }

    fn module_path(&self, spec: &Spec, module: &str) -> PathBuf {
        PathBuf::from(&spec.slug).join(format!("{}.mjs", naming::snake(module)))
    }

    fn stubs(&self, spec: &Spec) -> Vec<(PathBuf, String)> {
        let mut text = spec.main.summary.clone();
        if spec.main.fallible {
            text.push_str(" Throw an Error for invalid input.");
        }
        let mut main = String::from("import QtQuick\n\nQtObject {\n    id: root\n\n");
        if spec.concurrency {
            main.push_str(CLOCK);
        }
        main.push_str(&self.type_defs(spec));
        main.push_str(&format!(
            "{}    {} {{\n        throw new Error(\"{}\")\n    }}\n}}\n",
            doc("    // ", &text),
            self.signature(spec, &spec.main),
            stub_message(&self.function_name(&spec.main))
        ));
        let mut out = vec![(self.stub_path(spec), main)];
        for m in spec.modules() {
            let mut s = String::new();
            for (i, f) in spec.helpers.iter().filter(|h| h.module.as_deref() == Some(m)).enumerate() {
                if i > 0 {
                    s.push('\n');
                }
                s.push_str(&format!(
                    "{}{} {{\n    throw new Error(\"{}\")\n}}\n",
                    doc("// ", &f.summary),
                    self.signature(spec, f),
                    stub_message(&self.function_name(f))
                ));
            }
            out.push((self.module_path(spec, m), s));
        }
        out
    }

    fn tests(&self, spec: &Spec) -> Vec<(PathBuf, String)> {
        let comp = Self::component(spec);
        let mut s = header("//", spec, &format!("{comp}.qml"));
        s.push_str("\nimport QtQuick\nimport QtTest\n");
        for m in spec.modules() {
            s.push_str(&format!("import \"{}.mjs\" as {}\n", naming::snake(m), naming::pascal(m)));
        }
        s.push_str(&format!(
            "\nTestCase {{\n    name: \"{comp}\"\n\n    {comp} {{ id: subject }}\n\n\
             \x20   // The untouched stub throws this; it doesn't count as the expected error.\n\
             \x20   function isStub(e) {{\n        return e instanceof Error && String(e.message).startsWith(\"TODO: implement\")\n    }}\n"
        ));
        for (f, case) in spec.cases() {
            let name = case_name(case);
            match &case.kind {
                CaseKind::Call { input, outcome } => {
                    let call = self.call(spec, f, input);
                    match outcome {
                        Outcome::Value(v) if spec.concurrency => {
                            let t = timing(input);
                            s.push_str(&format!(
                                "\n    function test_{name}() {{\n        subject.now = 0\n        compare({call}, {})\n        const ticks = subject.now\n        verify(ticks >= {}, \"the clock only reached \" + ticks + \" ticks, but the slowest task needs {}: run every task to completion\")\n        verify(ticks <= {}, \"took \" + ticks + \" ticks: advance the generators round-robin so it takes about {} ticks, not {}\")\n    }}\n",
                                self.literal(spec, v, &f.returns),
                                t.max, t.max, t.max + 1, t.max, t.sum
                            ));
                        }
                        Outcome::Value(v) if kept_inputs(spec, f).iter().any(Option::is_some) => {
                            // Arrays and objects are shared with the caller. Compare
                            // JSON snapshots: class values carry their own methods,
                            // which compare() would see as different functions.
                            let kept = kept_inputs(spec, f);
                            let (args, vars) =
                                render_kept_args(f, input, &kept, |v, t| self.literal(spec, v, t), naming::camel);
                            s.push_str(&format!("\n    function test_{name}() {{\n"));
                            for (var, lit, _) in &vars {
                                s.push_str(&format!("        const {var} = {lit}\n        const {var}Before = JSON.stringify({var})\n"));
                            }
                            s.push_str(&format!(
                                "        compare(subject.{}({args}), {})\n",
                                self.function_name(f),
                                self.literal(spec, v, &f.returns)
                            ));
                            for (var, _, kind) in &vars {
                                s.push_str(&format!(
                                    "        compare(JSON.stringify({var}), {var}Before, \"the input {kind} was modified\")\n"
                                ));
                            }
                            s.push_str("    }\n");
                        }
                        Outcome::Value(v) => s.push_str(&format!(
                            "\n    function test_{name}() {{\n        compare({call}, {})\n    }}\n",
                            self.literal(spec, v, &f.returns)
                        )),
                        Outcome::Error => s.push_str(&format!(
                            "\n    function test_{name}() {{\n        let error = null\n        try {{\n            {call}\n        }} catch (e) {{\n            error = e\n        }}\n        verify(error !== null, \"expected an error to be thrown\")\n        verify(!isStub(error), \"still the stub: \" + error)\n    }}\n"
                        )),
                    }
                }
                CaseKind::Scenario { create, steps } => {
                    s.push_str(&format!("\n    function test_{name}() {{\n"));
                    for (i, args) in create.iter().enumerate() {
                        s.push_str(&format!("        const v{i} = {}\n", self.call(spec, f, args)));
                    }
                    let Type::Fn(ps, ret) = &f.returns else { unreachable!() };
                    for (n, step) in steps.iter().enumerate() {
                        let args: Vec<String> = step.input.iter().zip(ps).map(|(v, t)| self.literal(spec, v, t)).collect();
                        s.push_str(&format!(
                            "        compare(v{}({}), {}, \"{}\")\n",
                            step.on,
                            args.join(", "),
                            self.literal(spec, &step.expected, ret),
                            step_label(n + 1, step.on, create.len())
                        ));
                    }
                    s.push_str("    }\n");
                }
            }
        }
        s.push_str("}\n");
        vec![(PathBuf::from(&spec.slug).join(format!("tst_{}.qml", naming::snake(&spec.slug))), s)]
    }

    fn shared(&self, _specs: &[Spec]) -> Vec<(PathBuf, String)> {
        Vec::new()
    }

    fn command(&self, spec: &Spec, tool: &Found, track_dir: &Path) -> Invocation {
        // On Windows qmltestrunner's console output doesn't reach a pipe, so the
        // report goes to a file instead.
        let report = std::env::temp_dir().join(format!("learn-qml-{}-{}.txt", std::process::id(), spec.slug));
        Invocation::new(&tool.path, track_dir)
            .args(["-platform", "offscreen", "-input", spec.slug.as_str()])
            .args(["-o".to_string(), format!("{},txt", report.display())])
            .env("QT_QPA_PLATFORM", "offscreen")
            .output_file(report)
            .timeout(Duration::from_secs(60))
    }

    /// `FAIL!  : Comp::test_name() message`, plus following `Actual`/`Expected` lines.
    fn failure_reason(&self, output: &str, test_name: &str) -> Option<String> {
        let marker = format!("::test_{test_name}()");
        let mut lines = output.lines().skip_while(|l| !(l.starts_with("FAIL!") && l.contains(&marker)));
        let first = lines.next()?;
        let mut out = vec![first[first.find(&marker)? + marker.len()..].trim().to_string()];
        for l in lines.take_while(|l| l.starts_with("   ")) {
            if l.contains("Actual") || l.contains("Expected") {
                out.push(l.trim().to_string());
            }
        }
        Some(out.join("\n"))
    }

    fn parse_results(&self, output: &str) -> Vec<(usize, bool)> {
        output
            .lines()
            .filter_map(|line| {
                let t = line.trim();
                let passed = if t.starts_with("PASS") {
                    true
                } else if t.starts_with("FAIL!") {
                    false
                } else {
                    return None;
                };
                Some((naming::find_case_index(t)?, passed))
            })
            .collect()
    }
}
