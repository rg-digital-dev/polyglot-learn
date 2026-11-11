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
