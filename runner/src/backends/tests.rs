use serde_json::json;

use super::*;
use crate::spec::{Spec, Type};

fn ty(s: &str) -> Type {
    Type::parse(s).unwrap()
}

/// A spec with a record, an interface and a class, for rendering named types.
fn spec() -> Spec {
    Spec::parse(
        r#"{"slug":"sum-of-multiples","title":"T","concept":"loops","summary":"S.","function":"sum_of_multiples",
            "types":[{"name":"line_item","kind":"record","fields":[{"name":"sku","type":"string"},{"name":"unit_cents","type":"int"}]},
                     {"name":"shape","kind":"interface","methods":[{"name":"area","returns":"int"}]},
                     {"name":"rect","kind":"class","implements":["shape"],"fields":[{"name":"width","type":"int"},{"name":"height","type":"int"}]}],
            "params":[{"name":"factors","type":"list<int>"},{"name":"limit","type":"int"}],"returns":"int",
            "cases":[{"description":"d","input":[[3],10],"expected":18}]}"#,
    )
    .unwrap()
}

fn lit(id: &str, v: serde_json::Value, t: &str) -> String {
    get(id).unwrap().literal(&spec(), &v, &ty(t))
}

const ITEM: &str = r#"{"sku":"tea","unit_cents":150}"#;
const RECT: &str = r#"{"type":"rect","width":2,"height":3}"#;

fn val(s: &str) -> serde_json::Value {
    serde_json::from_str(s).unwrap()
}

#[test]
fn rust_literals_are_owned_and_typed() {
    assert_eq!(lit("rust", json!("hi"), "string"), "String::from(\"hi\")");
    assert_eq!(lit("rust", json!([1, -2]), "list<int>"), "vec![1, -2]");
    assert_eq!(lit("rust", json!([]), "list<int>"), "vec![]");
    assert_eq!(lit("rust", json!({}), "map<string,int>"), "HashMap::new()");
    assert_eq!(
        lit("rust", json!({"a": 1}), "map<string,int>"),
        "HashMap::from([(String::from(\"a\"), 1)])"
    );
    assert_eq!(lit("rust", json!(["a"]), "list<string>"), "vec![String::from(\"a\")]");
    assert_eq!(lit("rust", json!("tab\there \"q\""), "string"), "String::from(\"tab\\there \\\"q\\\"\")");
    let s = spec();
    let rust = get("rust").unwrap();
    assert_eq!(rust.type_name(&s, &ty("list<list<string>>"), true), "&[Vec<String>]");
    assert_eq!(rust.type_name(&s, &ty("list<shape>"), true), "&[Box<dyn Shape>]");
    assert_eq!(rust.type_name(&s, &ty("fn() -> int"), false), "impl FnMut() -> i64");
    assert_eq!(lit("rust", val(ITEM), "line_item"), "LineItem { sku: String::from(\"tea\"), unit_cents: 150 }");
    assert_eq!(lit("rust", val(RECT), "shape"), "Box::new(Rect { width: 2, height: 3 }) as Box<dyn Shape>");
}

#[test]
fn go_literals() {
    assert_eq!(lit("go", json!([1, 2]), "list<int>"), "[]int{1, 2}");
    assert_eq!(lit("go", json!([[1], []]), "list<list<int>>"), "[][]int{[]int{1}, []int{}}");
    assert_eq!(lit("go", json!({"a": true}), "map<string,bool>"), "map[string]bool{\"a\": true}");
    assert_eq!(lit("go", json!("\u{1}"), "string"), "\"\\x01\"");
    assert_eq!(lit("go", val(ITEM), "line_item"), "LineItem{Sku: \"tea\", UnitCents: 150}");
    assert_eq!(lit("go", val(&format!("[{RECT}]")), "list<shape>"), "[]Shape{Rect{Width: 2, Height: 3}}");
    assert_eq!(get("go").unwrap().type_name(&spec(), &ty("fn() -> int"), false), "func() int");
}

#[test]
fn php_literals() {
    assert_eq!(lit("php", json!({"a": [1]}), "map<string,list<int>>"), "[\"a\" => [1]]");
    assert_eq!(lit("php", json!("$x\n"), "string"), "\"\\$x\\n\"");
    assert_eq!(lit("php", json!(false), "bool"), "false");
    assert_eq!(lit("php", val(ITEM), "line_item"), "new LineItem(sku: \"tea\", unitCents: 150)");
    assert_eq!(lit("php", val(RECT), "shape"), "new Rect(width: 2, height: 3)");
}

#[test]
fn lua_literals() {
    assert_eq!(lit("lua", json!({"go": 3}), "map<string,int>"), "{[\"go\"] = 3}");
    assert_eq!(lit("lua", json!([]), "list<int>"), "{}");
    assert_eq!(lit("lua", json!("\tZoë\u{7}"), "string"), "\"\\tZoë\\007\"");
    assert_eq!(lit("lua", val(ITEM), "line_item"), "{ sku = \"tea\", unit_cents = 150 }");
    assert_eq!(lit("lua", val(RECT), "shape"), "M.Rect.new(2, 3)");
}

#[test]
fn js_family_literals() {
    assert_eq!(lit("typescript", json!({"a": [1, 2]}), "map<string,list<int>>"), "{\"a\": [1, 2]}");
    assert_eq!(lit("qml", json!({"a": [1, 2]}), "map<string,list<int>>"), "({\"a\": [1, 2]})");
    for id in ["typescript", "qml"] {
        assert_eq!(lit(id, json!("q\""), "string"), "\"q\\\"\"");
    }
    assert_eq!(lit("typescript", val(ITEM), "line_item"), "{ sku: \"tea\", unitCents: 150 }");
    assert_eq!(lit("typescript", val(RECT), "shape"), "new Rect(2, 3)");
    assert_eq!(lit("qml", val(RECT), "shape"), "subject.rect(2, 3)");
    assert_eq!(get("typescript").unwrap().type_name(&spec(), &ty("fn() -> int"), false), "() => number");
}

#[test]
fn function_names_follow_conventions() {
    let spec = spec();
    let names: Vec<String> = all().iter().map(|b| format!("{}={}", b.id(), b.function_name(&spec.main))).collect();
    assert_eq!(
        names,
        [
            "typescript=sumOfMultiples",
            "rust=sum_of_multiples",
            "go=SumOfMultiples",
            "php=sumOfMultiples",
            "lua=sum_of_multiples",
            "qml=sumOfMultiples"
        ]
    );
    assert_eq!(
        get("go").unwrap().stub_path(&spec),
        std::path::PathBuf::from("sum-of-multiples").join("sumofmultiples.go")
    );
    assert_eq!(
        get("lua").unwrap().stub_path(&spec),
        std::path::PathBuf::from("sum-of-multiples").join("sum_of_multiples.lua")
    );
    assert_eq!(
        get("rust").unwrap().signature(&spec, &spec.main),
        "pub fn sum_of_multiples(factors: &[i64], limit: i64) -> i64"
    );
    assert_eq!(get("go").unwrap().signature(&spec, &spec.main), "func SumOfMultiples(factors []int, limit int) int");
}

#[test]
fn every_backend_renders_every_repo_spec() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../specs");
    for entry in std::fs::read_dir(dir).unwrap() {
        let spec = Spec::load(&entry.unwrap().path()).unwrap();
        for b in all() {
            let stubs = b.stubs(&spec);
            assert_eq!(stubs[0].0, b.stub_path(&spec));
            assert_eq!(stubs.len(), 1 + spec.modules().len(), "{} {}", b.id(), spec.slug);
            let tests = b.tests(&spec);
            let text = &tests[0].1;
            for (_, case) in spec.cases() {
                assert!(text.contains(&case_name(case)), "{} {} is missing {}", b.id(), spec.slug, case_name(case));
            }
        }
    }
}

#[test]
fn parses_native_test_output() {
    let rust = get("rust").unwrap();
    assert_eq!(
        rust.parse_results("test c01_plain ... ok\ntest c02_x ... FAILED\ntest result: FAILED."),
        [(0, true), (1, false)]
    );
    let go = get("go").unwrap();
    assert_eq!(
        go.parse_results("    --- PASS: TestGreet/c01_a (0.00s)\n    --- FAIL: TestGreet/c02_b (0.00s)\n--- FAIL: TestGreet (0.00s)"),
        [(0, true), (1, false)]
    );
    let ts = get("typescript").unwrap();
    assert_eq!(ts.parse_results("ok 1 - c01_a\nnot ok 2 - c02_b\n# pass 1"), [(0, true), (1, false)]);
    let qml = get("qml").unwrap();
    assert_eq!(
        qml.parse_results("PASS   : Hello::initTestCase()\nPASS   : Hello::test_c01_a()\nFAIL!  : Hello::test_c02_b() x"),
        [(0, true), (1, false)]
    );
}

#[test]
fn aliases_resolve() {
    assert_eq!(get("ts").unwrap().id(), "typescript");
    assert_eq!(get("RS").unwrap().id(), "rust");
    assert!(get("cobol").is_err());
}
