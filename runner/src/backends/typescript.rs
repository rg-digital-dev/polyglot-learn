//! TypeScript on Node: `node --test` with built-in type stripping, no npm deps.

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::Value;

use super::{case_name, def_of, doc, field_values, header, json, kept_inputs, parse_tap, render_args, render_kept_args, step_label, stub_message, timing, Backend};
use crate::naming;
use crate::spec::{CaseKind, DefKind, Function, Method, Outcome, Param, Spec, Type};
use crate::toolchain::{parse_version, Found, Invocation, Tool};

pub struct TypeScript;

static TOOL: Tool = Tool {
    label: "node",
    candidates: &["node", "C:/Program Files/nodejs/node.exe", "/usr/local/bin/node", "/opt/homebrew/bin/node"],
    version_args: &["--version"],
    prefer: None,
};

const TASK: &str = "/** Provided, don't change: a slow task. Resolves to `ms` after `ms` milliseconds. */
export function task(ms: number): Promise<number> {
  return new Promise((resolve) => setTimeout(() => resolve(ms), ms));
}

";

/// Node strips types by default from 22.18 (22.x line) and 23.6; 22.6+ has it
/// behind `--experimental-strip-types`.
fn strip_types_flag(version: (u64, u64, u64)) -> Option<bool> {
    match version {
        (major, _, _) if major >= 24 => Some(false),
        (23, minor, _) => Some(minor < 6),
        (22, minor, _) if minor >= 18 => Some(false),
        (22, minor, _) if minor >= 6 => Some(true),
        _ => None,
    }
}

impl TypeScript {
    fn params(&self, spec: &Spec, params: &[Param]) -> String {
        params
            .iter()
            .map(|p| format!("{}: {}", naming::camel(&p.name), self.type_name(spec, &p.ty, true)))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn method_sig(&self, spec: &Spec, m: &Method) -> String {
        format!("{}({}): {}", naming::camel(&m.name), self.params(spec, &m.params), self.type_name(spec, &m.returns, false))
    }

    fn type_defs(&self, spec: &Spec) -> String {
        let mut s = String::new();
        for d in &spec.types {
            let name = naming::pascal(&d.name);
            match d.kind {
                DefKind::Interface => {
                    s.push_str(&format!("export interface {name} {{\n"));
                    for m in &d.methods {
                        s.push_str(&format!("  {};\n", self.method_sig(spec, m)));
                    }
                    s.push_str("}\n\n");
                }
                DefKind::Record => {
                    s.push_str(&format!("export interface {name} {{\n"));
                    for f in &d.fields {
                        s.push_str(&format!("  {}: {};\n", naming::camel(&f.name), self.type_name(spec, &f.ty, false)));
                    }
                    s.push_str("}\n\n");
                }
                DefKind::Class => {
                    let implements = if d.implements.is_empty() {
                        String::new()
                    } else {
                        let names: Vec<String> = d.implements.iter().map(|i| naming::pascal(i)).collect();
                        format!(" implements {}", names.join(", "))
                    };
                    s.push_str(&format!("export class {name}{implements} {{\n"));
                    for f in &d.fields {
                        s.push_str(&format!("  {}: {};\n", naming::camel(&f.name), self.type_name(spec, &f.ty, false)));
                    }
                    s.push_str(&format!("\n  constructor({}) {{\n", self.params(spec, &d.fields)));
                    for f in &d.fields {
                        let n = naming::camel(&f.name);
                        s.push_str(&format!("    this.{n} = {n};\n"));
                    }
                    s.push_str("  }\n");
                    for m in &d.methods {
                        s.push_str(&format!(
                            "\n  {} {{\n    throw new Error(\"{}\");\n  }}\n",
                            self.method_sig(spec, m),
                            stub_message(&format!("{name}.{}", naming::camel(&m.name)))
                        ));
                    }
                    s.push_str("}\n\n");
                }
            }
        }
        s
    }

    fn fn_stub(&self, spec: &Spec, f: &Function) -> String {
        let mut text = f.summary.clone();
        if f.fallible {
            text.push_str(" Throw an Error for invalid input.");
        }
        format!(
            "/**\n{} */\n{} {{\n  throw new Error(\"{}\");\n}}\n",
            doc(" * ", &text),
            self.signature(spec, f),
            stub_message(&self.function_name(f))
        )
    }

    fn call(&self, spec: &Spec, f: &Function, input: &[Value]) -> String {
        let path = match &f.module {
            Some(m) => format!("{}.{}", naming::camel(m), self.function_name(f)),
            None => self.function_name(f),
        };
        format!("{path}({})", render_args(&f.params, input, |v, t| self.literal(spec, v, t)))
    }
}

impl Backend for TypeScript {
    fn id(&self) -> &'static str {
        "typescript"
    }
    fn name(&self) -> &'static str {
        "TypeScript"
    }
    fn fence(&self) -> &'static str {
        "ts"
    }
    fn tool(&self) -> &'static Tool {
        &TOOL
    }

    fn function_name(&self, f: &Function) -> String {
        naming::camel(&f.name)
    }

    fn type_name(&self, spec: &Spec, ty: &Type, _param: bool) -> String {
        match ty {
            Type::Int => "number".into(),
            Type::Bool => "boolean".into(),
            Type::Str => "string".into(),
            Type::List(t) => match **t {
                Type::List(_) | Type::Map(_) | Type::Fn(..) => format!("Array<{}>", self.type_name(spec, t, false)),
                _ => format!("{}[]", self.type_name(spec, t, false)),
            },
            Type::Map(t) => format!("Record<string, {}>", self.type_name(spec, t, false)),
            Type::Named(n) => naming::pascal(n),
            Type::Fn(ps, r) => {
                let ps: Vec<String> =
                    ps.iter().enumerate().map(|(i, t)| format!("a{i}: {}", self.type_name(spec, t, false))).collect();
                format!("({}) => {}", ps.join(", "), self.type_name(spec, r, false))
            }
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
                format!("{{{}}}", parts.join(", "))
            }
            (Type::Named(n), _) => {
                let def = def_of(spec, n, value);
                let fields = field_values(def, value);
                if def.kind == DefKind::Class {
                    let args: Vec<String> = fields.into_iter().map(|(f, v)| self.literal(spec, v, &f.ty)).collect();
                    format!("new {}({})", naming::pascal(&def.name), args.join(", "))
                } else {
                    let parts: Vec<String> = fields
                        .into_iter()
                        .map(|(f, v)| format!("{}: {}", naming::camel(&f.name), self.literal(spec, v, &f.ty)))
                        .collect();
                    format!("{{ {} }}", parts.join(", "))
                }
            }
            // JSON is valid JS for scalars.
            _ => json(value),
        }
    }

    fn signature(&self, spec: &Spec, f: &Function) -> String {
        let ret = self.type_name(spec, &f.returns, false);
        let (asyncness, ret) = if spec.concurrency && f.module.is_none() {
            ("async ", format!("Promise<{ret}>"))
        } else {
            ("", ret)
        };
        format!("export {asyncness}function {}({}): {ret}", self.function_name(f), self.params(spec, &f.params))
    }

    fn stub_path(&self, spec: &Spec) -> PathBuf {
        PathBuf::from(&spec.slug).join(format!("{}.ts", spec.slug))
    }

    fn module_path(&self, spec: &Spec, module: &str) -> PathBuf {
        PathBuf::from(&spec.slug).join(format!("{}.ts", naming::kebab(module)))
    }

    fn stubs(&self, spec: &Spec) -> Vec<(PathBuf, String)> {
        let mut main = String::new();
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
        let mut s = header("//", spec, &format!("{}.ts", spec.slug));
        s.push_str("\nimport { test } from \"node:test\";\nimport assert from \"node:assert/strict\";\n");
        let mut names = vec![self.function_name(&spec.main)];
        names.extend(spec.types.iter().filter(|d| d.kind == DefKind::Class).map(|d| naming::pascal(&d.name)));
        s.push_str(&format!("import {{ {} }} from \"./{}.ts\";\n", names.join(", "), spec.slug));
        for m in spec.modules() {
            s.push_str(&format!("import * as {} from \"./{}.ts\";\n", naming::camel(m), naming::kebab(m)));
        }
        s.push_str(
            "\n// The untouched stub throws this; it doesn't count as the expected error.\n\
             const notStub = (e: unknown) => !(e instanceof Error && e.message.startsWith(\"TODO: implement\"));\n",
        );

        for (f, case) in spec.cases() {
            let name = case_name(case);
            match &case.kind {
                CaseKind::Call { input, outcome } => {
                    let call = self.call(spec, f, input);
                    match outcome {
                        Outcome::Value(v) if spec.concurrency => {
                            let t = timing(input);
                            s.push_str(&format!(
                                "\ntest(\"{name}\", async () => {{\n  const start = performance.now();\n  const got = await {call};\n  const elapsed = Math.round(performance.now() - start);\n  assert.deepStrictEqual(got, {});\n  assert.ok(elapsed >= {}, `finished in ${{elapsed}} ms, but the slowest task alone takes {} ms: call task() for every delay`);\n  assert.ok(elapsed < {}, `took ${{elapsed}} ms: run the tasks at the same time so the call takes about {} ms, not {} ms`);\n}});\n",
                                self.literal(spec, v, &f.returns),
                                t.min_ms, t.max, t.limit_ms, t.max, t.sum
                            ));
                        }
                        Outcome::Value(v) if kept_inputs(spec, f).iter().any(Option::is_some) => {
                            let kept = kept_inputs(spec, f);
                            let (args, vars) =
                                render_kept_args(f, input, &kept, |v, t| self.literal(spec, v, t), naming::camel);
                            let call = format!("{}({args})", self.function_name(f));
                            let call = if matches!(f.returns, Type::Map(_)) { format!("{{ ...{call} }}") } else { call };
                            s.push_str(&format!("\ntest(\"{name}\", () => {{\n"));
                            for (var, lit, _) in &vars {
                                // Typed from the function itself, so an empty `[]` isn't an
                                // implicit any[] under `strict` and no named type needs importing.
                                let i = f.params.iter().position(|p| naming::camel(&p.name) == *var).unwrap_or(0);
                                s.push_str(&format!(
                                    "  const {var}: Parameters<typeof {}>[{i}] = {lit};\n",
                                    self.function_name(f)
                                ));
                            }
                            s.push_str(&format!("  assert.deepStrictEqual({call}, {});\n", self.literal(spec, v, &f.returns)));
                            for (var, lit, kind) in &vars {
                                s.push_str(&format!("  assert.deepStrictEqual({var}, {lit}, \"the input {kind} was modified\");\n"));
                            }
                            s.push_str("});\n");
                        }
                        Outcome::Value(v) => {
                            // Spread maps into a plain object so Object.create(null) results compare equal.
                            let actual = if matches!(f.returns, Type::Map(_)) { format!("{{ ...{call} }}") } else { call };
                            s.push_str(&format!(
                                "\ntest(\"{name}\", () => {{\n  assert.deepStrictEqual({actual}, {});\n}});\n",
                                self.literal(spec, v, &f.returns)
                            ));
                        }
                        Outcome::Error => {
                            s.push_str(&format!("\ntest(\"{name}\", () => {{\n  assert.throws(() => {call}, notStub);\n}});\n"));
                        }
                    }
                }
                CaseKind::Scenario { create, steps } => {
                    s.push_str(&format!("\ntest(\"{name}\", () => {{\n"));
                    for (i, args) in create.iter().enumerate() {
                        s.push_str(&format!("  const v{i} = {};\n", self.call(spec, f, args)));
                    }
                    let Type::Fn(ps, ret) = &f.returns else { unreachable!() };
                    for (n, step) in steps.iter().enumerate() {
                        let args: Vec<String> = step.input.iter().zip(ps).map(|(v, t)| self.literal(spec, v, t)).collect();
                        s.push_str(&format!(
                            "  assert.deepStrictEqual(v{}({}), {}, \"{}\");\n",
                            step.on,
                            args.join(", "),
                            self.literal(spec, &step.expected, ret),
                            step_label(n + 1, step.on, create.len())
                        ));
                    }
                    s.push_str("});\n");
                }
            }
        }
        vec![(PathBuf::from(&spec.slug).join(format!("{}.test.ts", spec.slug)), s)]
    }

    /// Yours to change: `npm install --save-dev @types/node` adds to it.
    fn shared_scaffold(&self) -> Vec<(PathBuf, String)> {
        let package = "{\n  \"name\": \"learn-typescript\",\n  \"private\": true,\n  \"type\": \"module\",\n  \"description\": \"Created by learn. Tests run with node --test and need no dependencies. For type checking in your editor: npm install --save-dev @types/node\"\n}\n";
        vec![(PathBuf::from("package.json"), package.into())]
    }

    fn shared(&self, _specs: &[Spec]) -> Vec<(PathBuf, String)> {
        // Only for editors: Node itself ignores tsconfig and never type-checks.
        // The lib matches the oldest supported Node (22.6): ES2024 (toSorted,
        // Object.groupBy) plus iterator helpers. Both names need TypeScript 5.7+.
        // `types` names Node's own type definitions, an optional install
        // (TypeScript 6+ no longer picks up @types packages by itself).
        let tsconfig = "{\n  \"compilerOptions\": {\n    \"target\": \"ES2024\",\n    \"lib\": [\"ES2024\", \"ESNext.Iterator\"],\n    \"module\": \"NodeNext\",\n    \"moduleResolution\": \"NodeNext\",\n    \"strict\": true,\n    \"noEmit\": true,\n    \"allowImportingTsExtensions\": true,\n    \"verbatimModuleSyntax\": true,\n    \"erasableSyntaxOnly\": true,\n    \"types\": [\"node\"]\n  }\n}\n";
        vec![(PathBuf::from("tsconfig.json"), tsconfig.into())]
    }

    fn command(&self, spec: &Spec, tool: &Found, track_dir: &Path) -> Invocation {
        let mut args = vec!["--disable-warning=ExperimentalWarning".to_string()];
        if parse_version(&tool.version).and_then(strip_types_flag) == Some(true) {
            args.push("--experimental-strip-types".into());
        }
        args.extend(["--test".into(), "--test-reporter=tap".into()]);
        args.push(format!("{}/{}.test.ts", spec.slug, spec.slug));
        Invocation::new(&tool.path, track_dir).args(args).timeout(Duration::from_secs(60))
    }

    fn parse_results(&self, output: &str) -> Vec<(usize, bool)> {
        parse_tap(output)
    }

    /// The YAML block after `not ok N - name`: its `error:` value, which may span lines.
    fn failure_reason(&self, output: &str, test_name: &str) -> Option<String> {
        let block = super::lines_after(
            output,
            |l| {
                let t = l.trim();
                t.starts_with("not ok") && t.ends_with(&format!(" - {test_name}"))
            },
            |l| {
                let t = l.trim_start();
                t.starts_with("ok ") || t.starts_with("not ok ") || t.starts_with("# ")
            },
            40,
        )?;
        let mut lines = block.lines().skip_while(|l| !l.trim_start().starts_with("error:"));
        let first = lines.next()?.trim_start().trim_start_matches("error:").trim();
        let indent = |l: &str| l.len() - l.trim_start().len();
        let mut out = Vec::new();
        if !first.starts_with('|') {
            out.push(first.trim_matches('\'').to_string());
        }
        // Continuation lines are indented deeper than the next YAML key.
        let rest: Vec<&str> = lines.collect();
        let key_indent = rest.iter().filter(|l| l.trim_end().ends_with(':') || l.contains(": ")).map(|l| indent(l)).min();
        for l in rest {
            if key_indent.is_some_and(|k| indent(l) <= k) && (l.contains(": ") || l.trim_end().ends_with(':')) {
                break;
            }
            out.push(l.trim().to_string());
        }
        out.retain(|l| !l.is_empty());
        out.truncate(8);
        (!out.is_empty()).then(|| out.join("\n"))
    }

    fn check_tool(&self, tool: &Found) -> Option<String> {
        let v = parse_version(&tool.version)?;
        strip_types_flag(v)
            .is_none()
            .then(|| format!("Node 22.6 or newer is needed for TypeScript type stripping (found {})", tool.version))
    }
}

#[cfg(test)]
mod tests {
    use super::strip_types_flag;

    #[test]
    fn strip_flag_by_version() {
        assert_eq!(strip_types_flag((20, 11, 0)), None);
        assert_eq!(strip_types_flag((22, 5, 0)), None);
        assert_eq!(strip_types_flag((22, 6, 0)), Some(true));
        assert_eq!(strip_types_flag((22, 18, 0)), Some(false));
        assert_eq!(strip_types_flag((23, 5, 0)), Some(true));
        assert_eq!(strip_types_flag((23, 6, 0)), Some(false));
        assert_eq!(strip_types_flag((24, 0, 0)), Some(false));
    }
}
