//! Language-neutral exercise specs (`specs/<slug>.json`) and their validation.
//!
//! A spec describes one main function and, optionally, helper functions that
//! live in their own modules. Functions use a small type vocabulary: `int`,
//! `bool`, `string`, `list<T>`, `map<string,T>`, named types declared in
//! `types` (records, classes, interfaces), and `fn(..) -> T` as a return type.

use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::path::Path;

use serde::Deserialize;
use serde_json::Value;

use crate::bail;
use crate::error::{Context, Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Str,
    List(Box<Type>),
    Map(Box<Type>),
    /// A record, class or interface declared in the spec's `types`.
    Named(String),
    /// A function value: `fn(int) -> bool`. Only allowed as a return type.
    Fn(Vec<Type>, Box<Type>),
}

/// Split on top-level commas (ignoring commas inside `<...>` or `(...)`).
fn split_top(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let (mut depth, mut start) = (0i32, 0);
    for (i, c) in s.char_indices() {
        match c {
            '<' | '(' => depth += 1,
            '>' | ')' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(&s[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    if !s.is_empty() {
        parts.push(&s[start..]);
    }
    parts
}

impl Type {
    pub fn parse(src: &str) -> Result<Type> {
        let s: String = src.chars().filter(|c| !c.is_whitespace()).collect();
        Self::parse_compact(&s).ok_or_else(|| {
            Error(format!(
                "unknown type `{src}` (allowed: int, bool, string, list<T>, map<string,T>, fn(..) -> T, or a name from `types`)"
            ))
        })
    }

    fn parse_compact(s: &str) -> Option<Type> {
        match s {
            "int" => Some(Type::Int),
            "bool" => Some(Type::Bool),
            "string" => Some(Type::Str),
            _ => {
                if let Some(inner) = s.strip_prefix("list<").and_then(|r| r.strip_suffix('>')) {
                    Some(Type::List(Box::new(Self::parse_compact(inner)?)))
                } else if let Some(inner) = s.strip_prefix("map<string,").and_then(|r| r.strip_suffix('>')) {
                    Some(Type::Map(Box::new(Self::parse_compact(inner)?)))
                } else if let Some(rest) = s.strip_prefix("fn(") {
                    let close = rest.find(")->")?;
                    let params = split_top(&rest[..close])
                        .into_iter()
                        .map(Self::parse_compact)
                        .collect::<Option<Vec<_>>>()?;
                    let ret = Self::parse_compact(&rest[close + 3..])?;
                    Some(Type::Fn(params, Box::new(ret)))
                } else if is_snake_ident(s) {
                    Some(Type::Named(s.to_string()))
                } else {
                    None
                }
            }
        }
    }

    /// Does this type mention a map anywhere (directly or through record fields)?
    pub fn uses_map(&self, defs: &[TypeDef]) -> bool {
        self.any(defs, &mut |t| matches!(t, Type::Map(_)), &mut HashSet::new())
    }

    fn any(&self, defs: &[TypeDef], pred: &mut dyn FnMut(&Type) -> bool, seen: &mut HashSet<String>) -> bool {
        if pred(self) {
            return true;
        }
        match self {
            Type::List(t) | Type::Map(t) => t.any(defs, pred, seen),
            Type::Fn(ps, r) => ps.iter().any(|p| p.any(defs, pred, seen)) || r.any(defs, pred, seen),
            Type::Named(n) => {
                if !seen.insert(n.clone()) {
                    return false;
                }
                defs.iter().filter(|d| d.name == *n || d.implements.contains(n)).any(|d| {
                    d.fields.iter().any(|f| f.ty.any(defs, pred, seen))
                        || d.methods.iter().any(|m| m.returns.any(defs, pred, seen))
                })
            }
            _ => false,
        }
    }

    /// Does this type mention a named type for which `pred` holds?
    pub fn any_named(&self, defs: &[TypeDef], pred: &mut dyn FnMut(&str) -> bool) -> bool {
        self.any(defs, &mut |t| matches!(t, Type::Named(n) if pred(n)), &mut HashSet::new())
    }

    /// Check that a JSON value has this type. Returns a description of the
    /// first mismatch.
    pub fn check(&self, value: &Value, defs: &[TypeDef]) -> std::result::Result<(), String> {
        match (self, value) {
            (Type::Int, Value::Number(n)) if n.is_i64() => Ok(()),
            (Type::Bool, Value::Bool(_)) => Ok(()),
            (Type::Str, Value::String(_)) => Ok(()),
            (Type::List(inner), Value::Array(items)) => {
                for (i, item) in items.iter().enumerate() {
                    inner.check(item, defs).map_err(|e| format!("[{i}]: {e}"))?;
                }
                Ok(())
            }
            (Type::Map(inner), Value::Object(entries)) => {
                for (k, v) in entries {
                    inner.check(v, defs).map_err(|e| format!("[{k:?}]: {e}"))?;
                }
                Ok(())
            }
            (Type::Named(name), Value::Object(_)) => {
                let def = concrete_def(name, value, defs)?;
                let obj = value.as_object().expect("checked above");
                for f in &def.fields {
                    let v = obj.get(&f.name).ok_or_else(|| format!("{} is missing field `{}`", def.name, f.name))?;
                    f.ty.check(v, defs).map_err(|e| format!(".{}: {e}", f.name))?;
                }
                for k in obj.keys() {
                    if k != "type" && !def.fields.iter().any(|f| f.name == *k) {
                        return Err(format!("{} has no field `{k}`", def.name));
                    }
                }
                Ok(())
            }
            (Type::Fn(..), _) => Err("function values can't appear in test data".into()),
            _ => Err(format!("expected {self}, got {value}")),
        }
    }
}

/// The record or class a value of type `name` actually is. Interface-typed
/// values name their class with a `"type"` key.
pub fn concrete_def<'a>(name: &str, value: &Value, defs: &'a [TypeDef]) -> std::result::Result<&'a TypeDef, String> {
    let def = defs.iter().find(|d| d.name == name).ok_or_else(|| format!("unknown type `{name}`"))?;
    let tag = value.get("type").and_then(Value::as_str);
    match def.kind {
        DefKind::Interface => {
            let tag = tag.ok_or_else(|| format!("a `{name}` value needs a \"type\" naming its class"))?;
            let class = defs
                .iter()
                .find(|d| d.name == tag)
                .ok_or_else(|| format!("unknown class `{tag}`"))?;
            if !class.implements.iter().any(|i| i == name) {
                return Err(format!("`{tag}` doesn't implement `{name}`"));
            }
            Ok(class)
        }
        _ => match tag {
            Some(t) if t != name => Err(format!("expected a `{name}`, got \"type\": \"{t}\"")),
            _ => Ok(def),
        },
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => f.write_str("int"),
            Type::Bool => f.write_str("bool"),
            Type::Str => f.write_str("string"),
            Type::List(t) => write!(f, "list<{t}>"),
            Type::Map(t) => write!(f, "map<string,{t}>"),
            Type::Named(n) => f.write_str(n),
            Type::Fn(ps, r) => {
                let ps: Vec<String> = ps.iter().map(ToString::to_string).collect();
                write!(f, "fn({}) -> {r}", ps.join(", "))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefKind {
    /// Plain data, compared by value in tests.
    Record,
    /// Data plus behaviour (methods from the interfaces it implements).
    Class,
    /// Methods only.
    Interface,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub name: String,
    pub params: Vec<Param>,
    pub returns: Type,
}

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub name: String,
    pub kind: DefKind,
    pub fields: Vec<Param>,
    pub methods: Vec<Method>,
    pub implements: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Outcome {
    Value(Value),
    Error,
}

/// One call on a value returned by the function (a closure call).
#[derive(Debug, Clone)]
pub struct Step {
    /// Which created value to call (index into `create`).
    pub on: usize,
    pub input: Vec<Value>,
    pub expected: Value,
}

#[derive(Debug, Clone)]
pub enum CaseKind {
    /// Call the function once and check the result.
    Call { input: Vec<Value>, outcome: Outcome },
    /// Call the function once per `create` entry to get function values, then
    /// call those in order.
    Scenario { create: Vec<Vec<Value>>, steps: Vec<Step> },
}

#[derive(Debug, Clone)]
pub struct Case {
    /// Position across all of the spec's cases (helpers first), used for test names.
    pub index: usize,
    pub description: String,
    pub kind: CaseKind,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    /// `None` for the main function; helper functions live in their own module.
    pub module: Option<String>,
    pub summary: String,
    pub params: Vec<Param>,
    pub returns: Type,
    pub fallible: bool,
    pub cases: Vec<Case>,
}

#[derive(Debug, Clone)]
pub struct Spec {
    pub slug: String,
    pub title: String,
    pub concept: String,
    pub hints: Vec<String>,
    pub types: Vec<TypeDef>,
    pub helpers: Vec<Function>,
    pub main: Function,
    /// The main function runs `task(ms)` for each delay; tests also check timing.
    pub concurrency: bool,
}

// ---- raw JSON shapes ----------------------------------------------------------

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSpec {
    slug: String,
    title: String,
    concept: String,
    summary: String,
    function: String,
    params: Vec<RawParam>,
    returns: String,
    #[serde(default)]
    fallible: bool,
    #[serde(default)]
    hints: Vec<String>,
    cases: Vec<RawCase>,
    #[serde(default)]
    types: Vec<RawTypeDef>,
    #[serde(default)]
    helpers: Vec<RawHelper>,
    #[serde(default)]
    concurrency: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawHelper {
    module: String,
    function: String,
    summary: String,
    params: Vec<RawParam>,
    returns: String,
    #[serde(default)]
    fallible: bool,
    cases: Vec<RawCase>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTypeDef {
    name: String,
    kind: String,
    #[serde(default)]
    fields: Vec<RawParam>,
    #[serde(default)]
    methods: Vec<RawMethod>,
    #[serde(default)]
    implements: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMethod {
    name: String,
    #[serde(default)]
    params: Vec<RawParam>,
    returns: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawParam {
    name: String,
    #[serde(rename = "type")]
    ty: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCase {
    description: String,
    #[serde(default)]
    input: Option<Vec<Value>>,
    #[serde(default)]
    expected: Option<Value>,
    #[serde(default)]
    error: bool,
    #[serde(default)]
    create: Option<Vec<Vec<Value>>>,
    #[serde(default)]
    calls: Option<Vec<RawStep>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawStep {
    #[serde(default)]
    on: usize,
    #[serde(default)]
    input: Vec<Value>,
    expected: Value,
}

pub fn is_snake_ident(s: &str) -> bool {
    !s.is_empty()
        && s.starts_with(|c: char| c.is_ascii_lowercase())
        && s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        && !s.contains("__")
        && !s.ends_with('_')
}

fn is_slug(s: &str) -> bool {
    !s.is_empty()
        && s.starts_with(|c: char| c.is_ascii_lowercase())
        && s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !s.contains("--")
        && !s.ends_with('-')
}

fn parse_params(raw: Vec<RawParam>, what: &str) -> Result<Vec<Param>> {
    let mut params: Vec<Param> = Vec::new();
    for p in raw {
        if !is_snake_ident(&p.name) {
            bail!("{what}: `{}` must be snake_case", p.name);
        }
        if params.iter().any(|q| q.name == p.name) {
            bail!("{what}: `{}` is declared twice", p.name);
        }
        let ty = Type::parse(&p.ty).map_err(|e| Error(format!("{what} `{}`: {e}", p.name)))?;
        params.push(Param { name: p.name, ty });
    }
    Ok(params)
}

/// Every named type mentioned must be declared; fn types only as a return type.
fn check_type_refs(ty: &Type, defs: &[TypeDef], what: &str, allow_fn: bool) -> Result<()> {
    match ty {
        Type::Named(n) => {
            if !defs.iter().any(|d| d.name == *n) {
                bail!("{what}: unknown type `{n}` (declare it in `types`)");
            }
        }
        Type::List(t) | Type::Map(t) => check_type_refs(t, defs, what, false)?,
        Type::Fn(ps, r) => {
            if !allow_fn {
                bail!("{what}: fn types are only allowed as a function's return type");
            }
            for p in ps {
                check_type_refs(p, defs, what, false)?;
            }
            check_type_refs(r, defs, what, false)?;
        }
        _ => {}
    }
    Ok(())
}

fn parse_defs(raw: Vec<RawTypeDef>) -> Result<Vec<TypeDef>> {
    let mut defs: Vec<TypeDef> = Vec::new();
    for d in raw {
        if !is_snake_ident(&d.name) || ["int", "bool", "string", "list", "map", "fn"].contains(&d.name.as_str()) {
            bail!("type name `{}` must be snake_case and not a built-in type", d.name);
        }
        if defs.iter().any(|e| e.name == d.name) {
            bail!("type `{}` is declared twice", d.name);
        }
        let kind = match d.kind.as_str() {
            "record" => DefKind::Record,
            "class" => DefKind::Class,
            "interface" => DefKind::Interface,
            other => bail!("type `{}`: kind `{other}` must be record, class or interface", d.name),
        };
        let fields = parse_params(d.fields, &format!("type `{}` field", d.name))?;
        let mut methods = Vec::new();
        for m in d.methods {
            if !is_snake_ident(&m.name) {
                bail!("type `{}`: method `{}` must be snake_case", d.name, m.name);
            }
            let params = parse_params(m.params, &format!("method `{}.{}` param", d.name, m.name))?;
            let returns = Type::parse(&m.returns).map_err(|e| Error(format!("method `{}.{}`: {e}", d.name, m.name)))?;
            methods.push(Method { name: m.name, params, returns });
        }
        match kind {
            DefKind::Interface if !fields.is_empty() => bail!("interface `{}` can't have fields", d.name),
            DefKind::Record if !methods.is_empty() || !d.implements.is_empty() => {
                bail!("record `{}` can't have methods; use kind \"class\"", d.name)
            }
            _ => {}
        }
        if kind != DefKind::Interface && fields.is_empty() {
            bail!("type `{}` needs at least one field", d.name);
        }
        defs.push(TypeDef { name: d.name, kind, fields, methods, implements: d.implements });
    }
    // Classes inherit the methods of the interfaces they implement.
    for i in 0..defs.len() {
        for iface in defs[i].implements.clone() {
            let Some(idef) = defs.iter().find(|d| d.name == iface) else {
                bail!("`{}` implements unknown interface `{iface}`", defs[i].name);
            };
            if idef.kind != DefKind::Interface {
                bail!("`{}` implements `{iface}`, which isn't an interface", defs[i].name);
            }
            let inherited: Vec<Method> =
                idef.methods.iter().filter(|m| !defs[i].methods.iter().any(|o| o.name == m.name)).cloned().collect();
            defs[i].methods.extend(inherited);
        }
    }
    for d in &defs {
        for f in &d.fields {
            check_type_refs(&f.ty, &defs, &format!("type `{}` field `{}`", d.name, f.name), false)?;
        }
        for m in &d.methods {
            for p in &m.params {
                check_type_refs(&p.ty, &defs, &format!("method `{}.{}`", d.name, m.name), false)?;
            }
            check_type_refs(&m.returns, &defs, &format!("method `{}.{}`", d.name, m.name), false)?;
        }
    }
    Ok(defs)
}

struct FnParts {
    name: String,
    module: Option<String>,
    summary: String,
    params: Vec<RawParam>,
    returns: String,
    fallible: bool,
    cases: Vec<RawCase>,
}

fn parse_function(raw: FnParts, defs: &[TypeDef], next_index: &mut usize) -> Result<Function> {
    let what = format!("function `{}`", raw.name);
    if !is_snake_ident(&raw.name) {
        bail!("function `{}` must be snake_case", raw.name);
    }
    if raw.summary.trim().is_empty() {
        bail!("{what}: summary must not be empty");
    }
    let params = parse_params(raw.params, &format!("{what} param"))?;
    for p in &params {
        check_type_refs(&p.ty, defs, &format!("{what} param `{}`", p.name), false)?;
    }
    let returns = Type::parse(&raw.returns).map_err(|e| Error(format!("{what} returns: {e}")))?;
    check_type_refs(&returns, defs, &format!("{what} returns"), true)?;
    let returns_fn = matches!(returns, Type::Fn(..));
    if returns_fn && raw.fallible {
        bail!("{what}: a function returning a function can't be fallible");
    }
    if let Type::Named(n) = &returns {
        if defs.iter().any(|d| d.name == *n && d.kind != DefKind::Record) {
            bail!("{what}: only records can be returned (tests compare them by value)");
        }
    }

    if raw.cases.is_empty() {
        bail!("{what}: needs at least one case");
    }
    let check_input = |input: &[Value], at: &str| -> Result<()> {
        if input.len() != params.len() {
            bail!("{at}: has {} inputs but `{}` takes {} params", input.len(), raw.name, params.len());
        }
        for (value, param) in input.iter().zip(&params) {
            param.ty.check(value, defs).map_err(|e| Error(format!("{at}: input `{}`: {e}", param.name)))?;
        }
        Ok(())
    };

    let mut cases = Vec::new();
    for (i, c) in raw.cases.into_iter().enumerate() {
        let at = format!("{what} case {} ({:?})", i + 1, c.description);
        if c.description.trim().is_empty() {
            bail!("{what} case {} has no description", i + 1);
        }
        let kind = if returns_fn {
            let (Some(create), Some(calls)) = (c.create, c.calls) else {
                bail!("{at}: `{}` returns a function, so cases need `create` and `calls`", raw.name);
            };
            if c.input.is_some() || c.expected.is_some() || c.error {
                bail!("{at}: use `create` and `calls` instead of `input`/`expected`/`error`");
            }
            if create.is_empty() || calls.is_empty() {
                bail!("{at}: `create` and `calls` must not be empty");
            }
            for args in &create {
                check_input(args, &at)?;
            }
            let Type::Fn(fn_params, fn_ret) = &returns else { unreachable!() };
            let mut steps = Vec::new();
            for (j, s) in calls.into_iter().enumerate() {
                if s.on >= create.len() {
                    bail!("{at}: call {} uses `on: {}` but only {} value(s) are created", j + 1, s.on, create.len());
                }
                if s.input.len() != fn_params.len() {
                    bail!("{at}: call {} has {} inputs; the function takes {}", j + 1, s.input.len(), fn_params.len());
                }
                for (v, t) in s.input.iter().zip(fn_params) {
                    t.check(v, defs).map_err(|e| Error(format!("{at}: call {} input: {e}", j + 1)))?;
                }
                fn_ret.check(&s.expected, defs).map_err(|e| Error(format!("{at}: call {} expected: {e}", j + 1)))?;
                steps.push(Step { on: s.on, input: s.input, expected: s.expected });
            }
            CaseKind::Scenario { create, steps }
        } else {
            if c.create.is_some() || c.calls.is_some() {
                bail!("{at}: `create`/`calls` are only for functions that return a function");
            }
            let input = c.input.ok_or_else(|| Error(format!("{at}: needs `input`")))?;
            check_input(&input, &at)?;
            let outcome = match (c.expected, c.error) {
                (Some(_), true) => bail!("{at}: has both `expected` and `\"error\": true`"),
                (None, false) => bail!("{at}: needs `expected` or `\"error\": true`"),
                (None, true) => {
                    if !raw.fallible {
                        bail!("{at}: error cases are only allowed when `fallible` is true");
                    }
                    Outcome::Error
                }
                (Some(v), false) => {
                    returns.check(&v, defs).map_err(|e| Error(format!("{at}: expected: {e}")))?;
                    Outcome::Value(v)
                }
            };
            CaseKind::Call { input, outcome }
        };
        cases.push(Case { index: *next_index, description: c.description, kind });
        *next_index += 1;
    }
    // A stub that throws "passes" error cases, so the first case must expect a
    // value: then an untouched stub fails straight away.
    if matches!(cases[0].kind, CaseKind::Call { outcome: Outcome::Error, .. }) {
        bail!("{what}: the first case must expect a value, so an untouched stub fails straight away");
    }
    Ok(Function {
        name: raw.name,
        module: raw.module,
        summary: raw.summary,
        params,
        returns,
        fallible: raw.fallible,
        cases,
    })
}

impl Spec {
    pub fn load(path: &Path) -> Result<Spec> {
        let text = std::fs::read_to_string(path).context(|| format!("can't read {}", path.display()))?;
        Spec::parse(&text).map_err(|e| Error(format!("invalid spec {}: {e}", path.display())))
    }

    pub fn parse(text: &str) -> Result<Spec> {
        let raw: RawSpec = serde_json::from_str(text).map_err(|e| Error(e.to_string()))?;

        if !is_slug(&raw.slug) {
            bail!("slug `{}` must be kebab-case (a-z, 0-9, -)", raw.slug);
        }
        if !is_slug(&raw.concept) {
            bail!("concept `{}` must be kebab-case", raw.concept);
        }
        if raw.title.trim().is_empty() {
            bail!("title must not be empty");
        }
        let types = parse_defs(raw.types)?;

        let mut next = 0;
        let mut helpers = Vec::new();
        let mut modules = BTreeMap::new();
        for h in raw.helpers {
            if !is_snake_ident(&h.module) {
                bail!("helper module `{}` must be snake_case", h.module);
            }
            if modules.insert((h.module.clone(), h.function.clone()), ()).is_some() {
                bail!("helper `{}` is declared twice in module `{}`", h.function, h.module);
            }
            let f = parse_function(
                FnParts {
                    name: h.function,
                    module: Some(h.module),
                    summary: h.summary,
                    params: h.params,
                    returns: h.returns,
                    fallible: h.fallible,
                    cases: h.cases,
                },
                &types,
                &mut next,
            )?;
            if matches!(f.returns, Type::Fn(..)) {
                bail!("helper `{}`: only the main function may return a function", f.name);
            }
            helpers.push(f);
        }
        let main = parse_function(
            FnParts {
                name: raw.function,
                module: None,
                summary: raw.summary,
                params: raw.params,
                returns: raw.returns,
                fallible: raw.fallible,
                cases: raw.cases,
            },
            &types,
            &mut next,
        )?;
        if next > 99 {
            bail!("a spec can have at most 99 cases");
        }

        if raw.concurrency {
            let ok = main.params.len() == 1
                && main.params[0].ty == Type::List(Box::new(Type::Int))
                && main.returns == Type::List(Box::new(Type::Int))
                && !main.fallible
                && helpers.is_empty();
            if !ok {
                bail!("a concurrency spec's function must take one list<int> of delays and return list<int>");
            }
        }

        let spec = Spec {
            slug: raw.slug,
            title: raw.title,
            concept: raw.concept,
            hints: raw.hints,
            types,
            helpers,
            main,
            concurrency: raw.concurrency,
        };
        let mut seen = HashSet::new();
        for (_, c) in spec.cases() {
            if !seen.insert(crate::naming::ident(&c.description)) {
                bail!("two cases share the description {:?}", c.description);
            }
        }
        Ok(spec)
    }

    /// Helpers first (their tests come first), then the main function.
    pub fn functions(&self) -> impl Iterator<Item = &Function> {
        self.helpers.iter().chain(std::iter::once(&self.main))
    }

    /// Every case in test order, with the function it belongs to.
    pub fn cases(&self) -> Vec<(&Function, &Case)> {
        self.functions().flat_map(|f| f.cases.iter().map(move |c| (f, c))).collect()
    }

    pub fn def(&self, name: &str) -> Option<&TypeDef> {
        self.types.iter().find(|d| d.name == name)
    }

    /// Helper modules in declaration order (no duplicates).
    pub fn modules(&self) -> Vec<&str> {
        let mut out: Vec<&str> = Vec::new();
        for h in &self.helpers {
            let m = h.module.as_deref().expect("helpers have modules");
            if !out.contains(&m) {
                out.push(m);
            }
        }
        out
    }

    pub fn uses_map(&self) -> bool {
        self.functions().any(|f| f.returns.uses_map(&self.types) || f.params.iter().any(|p| p.ty.uses_map(&self.types)))
            || self.types.iter().any(|d| d.fields.iter().any(|f| f.ty.uses_map(&self.types)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(extra: &str, cases: &str) -> String {
        format!(
            r#"{{"slug":"demo","title":"Demo","concept":"errors","summary":"Demo.","function":"parse_it",
                "params":[{{"name":"input","type":"string"}}],"returns":"list<int>"{extra},
                "cases":[{cases}]}}"#
        )
    }

    #[test]
    fn parses_types() {
        assert_eq!(Type::parse("int").unwrap(), Type::Int);
        assert_eq!(
            Type::parse("map<string, list<int>>").unwrap(),
            Type::Map(Box::new(Type::List(Box::new(Type::Int))))
        );
        assert_eq!(Type::parse("list<list<string>>").unwrap().to_string(), "list<list<string>>");
        assert_eq!(Type::parse("fn() -> int").unwrap(), Type::Fn(vec![], Box::new(Type::Int)));
        assert_eq!(
            Type::parse("fn(int, map<string,int>) -> bool").unwrap().to_string(),
            "fn(int, map<string,int>) -> bool"
        );
        assert_eq!(Type::parse("list<line_item>").unwrap(), Type::List(Box::new(Type::Named("line_item".into()))));
        assert!(Type::parse("float").is_ok(), "unknown names parse as named types; validation rejects them later");
        assert!(Type::parse("map<int,int>").is_err());
        assert!(Type::parse("list<int").is_err());
        assert!(Type::parse("Float").is_err());
    }

    #[test]
    fn checks_values_against_types() {
        let t = Type::parse("map<string,int>").unwrap();
        assert!(t.check(&serde_json::json!({"a": 1}), &[]).is_ok());
        assert!(t.check(&serde_json::json!({}), &[]).is_ok());
        assert!(t.check(&serde_json::json!({"a": "1"}), &[]).is_err());
        assert!(Type::Int.check(&serde_json::json!(1.5), &[]).is_err());
        assert!(Type::parse("list<int>").unwrap().check(&serde_json::json!([1, true]), &[]).is_err());
    }

    #[test]
    fn accepts_a_valid_spec() {
        let s = Spec::parse(&spec(
            r#","fallible":true,"hints":["h"]"#,
            r#"{"description":"ok","input":["1,2"],"expected":[1,2]},{"description":"bad","input":["x"],"error":true}"#,
        ))
        .unwrap();
        assert_eq!(s.cases().len(), 2);
        assert!(matches!(s.main.cases[1].kind, CaseKind::Call { outcome: Outcome::Error, .. }));
    }

    #[test]
    fn rejects_input_count_mismatch() {
        let err = Spec::parse(&spec("", r#"{"description":"x","input":[],"expected":[]}"#)).unwrap_err();
        assert!(err.0.contains("inputs"), "{err}");
    }

    #[test]
    fn rejects_wrong_input_type() {
        let err = Spec::parse(&spec("", r#"{"description":"x","input":[5],"expected":[]}"#)).unwrap_err();
        assert!(err.0.contains("input `input`"), "{err}");
    }

    #[test]
    fn rejects_wrong_expected_type() {
        let err = Spec::parse(&spec("", r#"{"description":"x","input":["a"],"expected":"no"}"#)).unwrap_err();
        assert!(err.0.contains("expected"), "{err}");
    }

    #[test]
    fn rejects_error_case_when_not_fallible() {
        let err = Spec::parse(&spec(
            "",
            r#"{"description":"a","input":["a"],"expected":[]},{"description":"b","input":["b"],"error":true}"#,
        ))
        .unwrap_err();
        assert!(err.0.contains("fallible"), "{err}");
    }

    #[test]
    fn rejects_case_without_outcome() {
        let err = Spec::parse(&spec("", r#"{"description":"x","input":["a"]}"#)).unwrap_err();
        assert!(err.0.contains("needs `expected`"), "{err}");
    }

    #[test]
    fn rejects_error_first() {
        let err = Spec::parse(&spec(
            r#","fallible":true"#,
            r#"{"description":"b","input":["b"],"error":true},{"description":"a","input":["a"],"expected":[]}"#,
        ))
        .unwrap_err();
        assert!(err.0.contains("first case"), "{err}");
    }

    #[test]
    fn rejects_unknown_fields_and_bad_names() {
        assert!(Spec::parse(&spec(r#","extra":1"#, r#"{"description":"x","input":["a"],"expected":[]}"#)).is_err());
        let bad = spec("", r#"{"description":"x","input":["a"],"expected":[]}"#).replace("parse_it", "parseIt");
        assert!(Spec::parse(&bad).unwrap_err().0.contains("snake_case"));
    }

    const SHAPES: &str = r#"{"slug":"s","title":"S","concept":"interfaces","summary":"S.",
        "types":[{"name":"shape","kind":"interface","methods":[{"name":"area","returns":"int"}]},
                 {"name":"sq","kind":"class","implements":["shape"],"fields":[{"name":"side","type":"int"}]},
                 {"name":"pt","kind":"record","fields":[{"name":"x","type":"int"}]}],
        "function":"total","params":[{"name":"shapes","type":"list<shape>"}],"returns":"int",
        "cases":[CASES]}"#;

    #[test]
    fn checks_named_types() {
        let ok = SHAPES.replace("CASES", r#"{"description":"a","input":[[{"type":"sq","side":2}]],"expected":4}"#);
        let s = Spec::parse(&ok).unwrap();
        let sq = s.def("sq").unwrap();
        assert_eq!(sq.methods.len(), 1, "classes inherit interface methods");

        for (bad, msg) in [
            (r#"[[{"side":2}]]"#, "needs a \"type\""),
            (r#"[[{"type":"pt","x":1}]]"#, "doesn't implement"),
            (r#"[[{"type":"sq"}]]"#, "missing field"),
            (r#"[[{"type":"sq","side":2,"extra":1}]]"#, "no field"),
            (r#"[[{"type":"nope","side":2}]]"#, "unknown class"),
        ] {
            let text = SHAPES.replace("CASES", &format!(r#"{{"description":"a","input":{bad},"expected":4}}"#));
            let err = Spec::parse(&text).unwrap_err();
            assert!(err.0.contains(msg), "{bad}: {err}");
        }
        let unknown = SHAPES.replace("list<shape>", "list<circle>").replace("CASES", r#"{"description":"a","input":[[]],"expected":0}"#);
        assert!(Spec::parse(&unknown).unwrap_err().0.contains("unknown type `circle`"));
    }

    #[test]
    fn checks_scenarios() {
        let base = r#"{"slug":"c","title":"C","concept":"closures","summary":"C.","function":"make",
            "params":[{"name":"start","type":"int"}],"returns":"fn() -> int","cases":[CASES]}"#;
        let ok = base.replace("CASES", r#"{"description":"a","create":[[1],[5]],"calls":[{"expected":1},{"on":1,"expected":5}]}"#);
        let s = Spec::parse(&ok).unwrap();
        assert!(matches!(&s.main.cases[0].kind, CaseKind::Scenario { steps, .. } if steps.len() == 2));

        for (bad, msg) in [
            (r#"{"description":"a","input":[1],"expected":1}"#, "need `create` and `calls`"),
            (r#"{"description":"a","create":[[1]],"calls":[{"on":1,"expected":1}]}"#, "only 1 value"),
            (r#"{"description":"a","create":[[1]],"calls":[{"expected":"x"}]}"#, "expected"),
            (r#"{"description":"a","create":[["x"]],"calls":[{"expected":1}]}"#, "input `start`"),
        ] {
            let err = Spec::parse(&base.replace("CASES", bad)).unwrap_err();
            assert!(err.0.contains(msg), "{bad}: {err}");
        }
    }

    #[test]
    fn numbers_cases_across_helpers() {
        let text = r#"{"slug":"m","title":"M","concept":"modules","summary":"M.",
            "helpers":[{"module":"words","function":"split","summary":"S.","params":[{"name":"t","type":"string"}],
                        "returns":"list<string>","cases":[{"description":"h1","input":["a"],"expected":["a"]}]}],
            "function":"slug","params":[{"name":"t","type":"string"}],"returns":"string",
            "cases":[{"description":"m1","input":["a"],"expected":"a"},{"description":"m2","input":["b"],"expected":"b"}]}"#;
        let s = Spec::parse(text).unwrap();
        let idx: Vec<(String, usize)> = s.cases().iter().map(|(f, c)| (f.name.clone(), c.index)).collect();
        assert_eq!(idx, [("split".into(), 0), ("slug".into(), 1), ("slug".into(), 2)]);
        assert_eq!(s.modules(), ["words"]);
    }

    #[test]
    fn repo_specs_are_valid() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../specs");
        let mut n = 0;
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "json") {
                let s = Spec::load(&path).unwrap();
                assert_eq!(path.file_stem().unwrap().to_str().unwrap(), s.slug);
                n += 1;
            }
        }
        assert!(n >= 12);
    }
}
