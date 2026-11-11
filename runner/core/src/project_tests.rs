//! Projects end to end, on a small course of their own: one Go project with
//! two stages that grow one package.

use std::path::{Path, PathBuf};

use crate::backends;
use crate::check;
use crate::ctx::Ctx;
use crate::lessons;

fn write(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

/// A course with an empty shared module and a Go path that's one project.
fn course(name: &str) -> (PathBuf, PathBuf) {
    let base = std::env::temp_dir().join(format!("learn-project-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let (root, ws) = (base.join("course"), base.join("ws"));
    write(&root.join("course.toml"), "title = \"t\"\n\n[[module]]\nid = \"basics\"\ntitle = \"Basics\"\nexercises = []\n");
    let real = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    write(&root.join("tracks/go/track.toml"), &std::fs::read_to_string(real.join("tracks/go/track.toml")).unwrap());
    write(
        &root.join("tracks/go/path.toml"),
        "[[stage]]\nid = \"counter\"\ntitle = \"Project: Counter\"\nproject = true\nexercises = [\"counter-add\", \"counter-double\"]\n",
    );
    let stage = |slug: &str, title: &str, stub: &str, cases: &[&str]| {
        let dir = root.join("tracks/go/extras").join(slug);
        let cases = cases.iter().map(|c| format!("{c:?}")).collect::<Vec<_>>().join(", ");
        write(
            &dir.join("exercise.toml"),
            &format!("title = \"{title}\"\nconcept = \"structs\"\nsummary = \"S.\"\nstubs = [\"{stub}\"]\nhints = [\"h\"]\ncases = [{cases}]\n"),
        );
        write(&dir.join("lesson.md"), &format!("# {title} lesson\n"));
        dir.join("files")
    };
    let one = stage("counter-add", "Add", "counter.go", &["starts at zero", "add counts up"]);
    write(&one.join("counter.go"), "package counter\n\ntype Counter struct{ n int }\n\nfunc (c *Counter) Add() { panic(\"TODO\") }\n\nfunc (c *Counter) Value() int { return c.n }\n");
    write(
        &one.join("counter_test.go"),
        "package counter\n\nimport \"testing\"\n\nfunc TestAdd(t *testing.T) {\n\tt.Run(\"c01_starts_at_zero\", func(t *testing.T) {\n\t\tvar c Counter\n\t\tif c.Value() != 0 {\n\t\t\tt.Fatal(c.Value())\n\t\t}\n\t})\n\tt.Run(\"c02_add_counts_up\", func(t *testing.T) {\n\t\tvar c Counter\n\t\tc.Add()\n\t\tc.Add()\n\t\tif c.Value() != 2 {\n\t\t\tt.Fatal(c.Value())\n\t\t}\n\t})\n}\n",
    );
    let two = stage("counter-double", "Double", "double.go", &["double doubles"]);
    write(&two.join("double.go"), "package counter\n\nfunc (c *Counter) Double() { panic(\"TODO\") }\n");
    write(
        &two.join("double_test.go"),
        "package counter\n\nimport \"testing\"\n\nfunc TestDouble(t *testing.T) {\n\tt.Run(\"c03_double_doubles\", func(t *testing.T) {\n\t\tvar c Counter\n\t\tc.Add()\n\t\tc.Double()\n\t\tif c.Value() != 2 {\n\t\t\tt.Fatal(c.Value())\n\t\t}\n\t})\n}\n",
    );
    (root, ws)
}

fn ctx(root: &Path, ws: &Path) -> Ctx {
    let mut ctx = Ctx::discover_from(root, Some(ws.to_path_buf())).unwrap();
    ctx.config.track = Some("go".into());
    ctx
}

#[test]
fn stages_share_a_folder_and_their_cases_build_up() {
    let (root, ws) = course("cases");
    let ctx = ctx(&root, &ws);
    let (one, two) = (ctx.spec("counter-add").unwrap(), ctx.spec("counter-double").unwrap());
    assert_eq!(one.dir("go"), PathBuf::from("01-counter"));
    assert_eq!(two.dir("go"), one.dir("go"), "one codebase, one folder");
    assert_eq!(two.project.as_ref().unwrap().number, 2);
    let cases: Vec<(usize, String)> = two.cases().iter().map(|(_, c)| (c.index, c.description.clone())).collect();
    assert_eq!(cases, [(0, "starts at zero".into()), (1, "add counts up".into()), (2, "double doubles".into())]);
    assert_eq!(one.cases().len(), 2);
}

#[test]
fn later_stages_arrive_when_the_one_before_passes() {
    let (root, ws) = course("arrive");
    let mut ctx = ctx(&root, &ws);
    let go = backends::get("go").unwrap();
    lessons::generate_track(&ctx, go).unwrap();
    let dir = ctx.exercise_dir("go", "counter-add");
    assert!(dir.join("counter_test.go").is_file());
    assert!(!dir.join("double.go").exists(), "stage 2 waits");
    let lesson = std::fs::read_to_string(dir.join(lessons::FILE)).unwrap();
    assert!(lesson.contains("**Project: Counter** · stage 1 of 2"), "{lesson}");
    assert_eq!(ctx.exercise_at("go", Path::new("01-counter/counter.go")).unwrap().slug, "counter-add");

    std::fs::write(dir.join("counter.go"), "package counter // mine\n").unwrap();
    check::record_pass(&mut ctx, "go", "counter-add").unwrap();
    assert!(dir.join("double.go").is_file() && dir.join("double_test.go").is_file());
    assert_eq!(std::fs::read_to_string(dir.join("counter.go")).unwrap(), "package counter // mine\n", "your code stays");
    assert!(std::fs::read_to_string(dir.join(lessons::FILE)).unwrap().contains("stage 2 of 2"));
    assert_eq!(ctx.exercise_at("go", Path::new("01-counter/counter.go")).unwrap().slug, "counter-double");
}

#[test]
fn checking_a_stage_runs_every_test_so_far() {
    let Some(tool) = backends::get("go").unwrap().tool().find() else { return }; // no Go here
    let (root, ws) = course("check");
    let mut ctx = ctx(&root, &ws);
    let go = backends::get("go").unwrap();
    lessons::generate_track(&ctx, go).unwrap();
    let dir = ctx.exercise_dir("go", "counter-add");
    // A check writes the specs it's given: the workspace's leave stage 2 out.
    let one = ctx.spec("counter-add").unwrap().clone();
    check::run(go, &one, &ctx.workspace_specs("go"), &ws.join("go"), &tool).unwrap();
    assert!(!dir.join("double_test.go").exists(), "checking stage 1 brought stage 2's tests");
    check::record_pass(&mut ctx, "go", "counter-add").unwrap();
    std::fs::write(dir.join("counter.go"), "package counter\n\ntype Counter struct{ n int }\n\nfunc (c *Counter) Add() { c.n++ }\n\nfunc (c *Counter) Value() int { return c.n }\n").unwrap();
    let specs = ctx.workspace_specs("go");
    let two = ctx.spec("counter-double").unwrap().clone();
    let r = check::run(go, &two, &specs, &ws.join("go"), &tool).unwrap();
    assert_eq!(r.cases.iter().map(|c| c.passed).collect::<Vec<_>>(), [Some(true), Some(true), Some(false)], "{}", r.output);

    std::fs::write(dir.join("double.go"), "package counter\n\nfunc (c *Counter) Double() { c.n *= 2 }\n").unwrap();
    let r = check::run(go, &two, &specs, &ws.join("go"), &tool).unwrap();
    assert!(r.passed, "{}", r.output);
}

#[test]
fn a_rust_project_is_one_crate_tested_as_a_whole() {
    let ws = std::env::temp_dir().join(format!("learn-project-rust-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&ws);
    let mut ctx = Ctx::discover(Some(ws.clone())).unwrap();
    ctx.config.track = Some("rust".into());
    let rust = backends::get("rust").unwrap();
    // Every stage at once, as the selftest's reference run has them.
    let specs: Vec<_> = ctx.track_specs("rust").into_iter().filter(|s| s.project.is_some()).collect();
    assert_eq!(specs.len(), 8, "the log CLI has eight stages");
    let dir = specs[0].dir("rust");
    backends::generate(rust, &specs, &ws.join("rust")).unwrap();
    let manifest = std::fs::read_to_string(ws.join("rust/Cargo.toml")).unwrap();
    assert_eq!(manifest.matches("log-cli\"").count(), 1, "the crate is a member once: {manifest}");
    let lib = std::fs::read_to_string(ws.join("rust").join(&dir).join("src/lib.rs")).unwrap();
    assert!(lib.contains("pub mod borrowed;"), "the last stage's lib.rs declares every module: {lib}");

    let tool = crate::toolchain::Found { path: "cargo".into(), version: String::new() };
    let run = rust.command(&specs[2], &tool, &ws.join("rust"));
    assert_eq!(run.args[..5], ["test", "-p", "log-cli", "--tests", "--no-fail-fast"], "every stage's test file runs");
}

#[test]
fn a_python_project_runs_every_stage_file_through_the_harness() {
    let ws = std::env::temp_dir().join(format!("learn-project-python-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&ws);
    let ctx = Ctx::discover(Some(ws.clone())).unwrap();
    let python = backends::get("python").unwrap();
    let specs: Vec<_> = ctx.track_specs("python").into_iter().filter(|s| s.project.is_some()).collect();
    assert_eq!(specs.len(), 8, "the data pipeline has eight stages");
    let tool = crate::toolchain::Found { path: "python".into(), version: String::new() };
    let run = python.command(&specs[3], &tool, &ws.join("python"));
    assert_eq!(run.args, ["-X", "utf8", "-B", "harness.py", specs[0].rel("python").as_str()], "the harness runs the folder");

    // The harness runs each test file in name order and fails if any did.
    let Some(tool) = python.tool().find() else { return }; // no Python here
    let dir = ws.join("python");
    backends::generate(python, &specs[..0], &dir).unwrap(); // writes harness.py
    let project = dir.join("demo");
    std::fs::create_dir_all(&project).unwrap();
    let test = |n: u32, ok: bool| {
        format!("import os, sys\nsys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))\nfrom harness import H\nt = H()\nt.equals('c0{n}_x', lambda: 1, {})\nt.finish()\n", if ok { 1 } else { 2 })
    };
    std::fs::write(project.join("test_s1_a.py"), test(1, true)).unwrap();
    std::fs::write(project.join("test_s2_b.py"), test(2, false)).unwrap();
    std::fs::write(project.join("test_s3_c.py"), "raise ImportError('broken stub')\n").unwrap();
    let out = std::process::Command::new(&tool.path).args(["-X", "utf8", "-B", "harness.py", "demo"]).current_dir(&dir).output().unwrap();
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(python.parse_results(&text), [(0, true), (1, false)], "{text}");
    assert!(text.contains("Bail out! test_s3_c.py didn't load: ImportError: broken stub"), "{text}");
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn a_typescript_project_type_checks_each_stage_on_its_own() {
    let ws = std::env::temp_dir().join(format!("learn-project-ts-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&ws);
    let ctx = Ctx::discover(Some(ws.clone())).unwrap();
    let ts = backends::get("typescript").unwrap();
    let specs: Vec<_> = ctx.track_specs("typescript").into_iter().filter(|s| s.project.is_some()).collect();
    assert_eq!(specs.len(), 8, "the task tracker has eight stages");

    // Stage 6 covers the type checks of stages 1, 2, 4 and 6, each filling its own first case.
    let stages = check::typecheck_stages(&specs[5], &specs);
    let firsts: Vec<(usize, &str)> = stages.iter().map(|(i, s)| (*i, s.slug.as_str())).collect();
    assert_eq!(firsts, [(0, "tracker-tasks"), (7, "tracker-events"), (19, "tracker-store"), (31, "tracker-query")]);
    for (i, s) in &stages {
        assert_eq!(specs[5].cases()[*i].1.description, "types check", "{} starts with its type check", s.slug);
    }
    // Stage 3 only has the first two so far.
    assert_eq!(check::typecheck_stages(&specs[2], &specs).len(), 2);
    // An exercise outside a project: its own, at case 0.
    let branded = ctx.spec_on("typescript", "branded-ids").unwrap();
    assert_eq!(check::typecheck_stages(branded, &specs).iter().map(|(i, _)| *i).collect::<Vec<_>>(), [0]);

    let tool = crate::toolchain::Found { path: "node".into(), version: String::new() };
    let run = ts.command(&specs[3], &tool, &ws.join("typescript"));
    assert_eq!(run.args.last().unwrap(), &format!("{}/*.test.ts", specs[0].rel("typescript")), "every stage's tests run");
}

#[test]
fn a_php_project_runs_each_stage_file_in_its_own_process() {
    let ws = std::env::temp_dir().join(format!("learn-project-php-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&ws);
    let ctx = Ctx::discover(Some(ws.clone())).unwrap();
    let php = backends::get("php").unwrap();
    let specs: Vec<_> = ctx.track_specs("php").into_iter().filter(|s| s.project.is_some()).collect();
    assert_eq!(specs.len(), 8, "the web framework has eight stages");
    let tool = crate::toolchain::Found { path: "php".into(), version: String::new() };
    let run = php.command(&specs[4], &tool, &ws.join("php"));
    assert_eq!(run.args, ["harness.php", specs[0].rel("php").as_str()], "the harness runs the folder");

    // Each file runs in its own process: both declare helper(), and the first exits.
    let Some(tool) = php.tool().find() else { return }; // no PHP here
    let dir = ws.join("php");
    backends::generate(php, &specs[..0], &dir).unwrap(); // writes harness.php
    let project = dir.join("demo");
    std::fs::create_dir_all(&project).unwrap();
    let test = |n: u32, ok: bool| {
        format!("<?php\nrequire __DIR__ . '/../harness.php';\nfunction helper(): int {{ return 1; }}\n$t = new Harness();\n$t->equals('c0{n}_x', fn() => helper(), {});\n$t->finish();\n", if ok { 1 } else { 2 })
    };
    std::fs::write(project.join("s1-a.test.php"), test(1, true)).unwrap();
    std::fs::write(project.join("s2-b.test.php"), test(2, false)).unwrap();
    std::fs::write(project.join("s3-c.test.php"), "<?php\nnew NoSuchClass();\n").unwrap();
    let out = std::process::Command::new(&tool.path).args(["harness.php", "demo"]).current_dir(&dir).output().unwrap();
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(php.parse_results(&text), [(0, true), (1, false)], "{text}");
    assert!(text.contains("Bail out! s3-c.test.php stopped with a fatal error"), "{text}");
    assert_eq!(out.status.code(), Some(1));
}
