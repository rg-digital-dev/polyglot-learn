# Stage 8 of the data pipeline project. Don't edit this file: `learn`
# overwrites it. Your code goes in cli.py.

import os
import sys

# This file's folder (the project) and the track folder (harness.py).
DIR = os.path.dirname(os.path.abspath(__file__))
sys.path[:0] = [DIR, os.path.dirname(DIR)]

from harness import H

import contextlib
import io
import subprocess
import tempfile

from cli import main

t = H()

TMP = tempfile.mkdtemp(prefix="pipeline-s8-")
CSV = (
    "order_id,date,customer,product,category,quantity,unit_price,country\n"
    "S1,2026-01-10,Ada,Pen,stationery,2,1.50,NL\n"
    "S2,2026-02-11,Bob,Chess,games,1,40.00,DE\n"
    "S3,oops,Cy,Ink,stationery,1,4.00,DE\n"
)
JSONL = '{"order_id": "W1", "date": "2026-02-20", "customer": "Di", "product": "Lamp", "category": "home", "quantity": 1, "unit_price": "25.00", "country": "FR"}\n'


def file(name, text):
    path = os.path.join(TMP, name)
    with open(path, "w", encoding="utf-8", newline="") as f:
        f.write(text)
    return path


def run(*argv):
    """main(argv), returning (exit code, stdout, stderr)."""
    out, err = io.StringIO(), io.StringIO()
    with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
        code = main(list(argv))
    return code, out.getvalue(), err.getvalue()


def db(name):
    return os.path.join(TMP, name)


GOOD = file("good.csv", CSV.rsplit("S3", 1)[0])
MIXED = file("mixed.csv", CSV)
WEB = file("web.jsonl", JSONL)


def case_41():
    code, out, err = run("import", GOOD, WEB, "--db", db("a.sqlite"))
    H.expect((code, err), (0, ""), "exit code and stderr")
    H.expect(out, "imported 3 new orders from 2 files (0 rows skipped)\n", "stdout")


t.scenario("c41_import_saves_orders_and_says_how_many", case_41)


def case_42():
    code, out, err = run("import", MIXED, "--db", db("b.sqlite"))
    H.expect(code, 1, "exit code when rows were skipped")
    H.check(err.startswith("mixed.csv:4: ") and "date" in err, f"stderr should name the file, line and problem: {err!r}")
    H.expect(out, "imported 2 new orders from 1 files (1 rows skipped)\n", "the good rows are still imported")


t.scenario("c42_bad_rows_go_to_stderr_and_set_the_exit_code", case_42)


def case_43():
    run("import", GOOD, "--db", db("c.sqlite"))
    code, out, _ = run("import", GOOD, WEB, "--db", db("c.sqlite"))
    H.expect((code, out), (0, "imported 1 new orders from 2 files (0 rows skipped)\n"), "only the new one")


t.scenario("c43_importing_twice_adds_only_new_orders", case_43)


def case_44():
    run("import", GOOD, WEB, "--db", db("d.sqlite"))
    code, out, _ = run("report", "--db", db("d.sqlite"), "--from", "2026-02-01")
    H.expect(code, 0, "exit code")
    H.check(out.startswith("# Sales report\n"), f"Markdown on stdout: {out[:60]!r}")
    H.check("· 2 orders · revenue 65.00" in out, "only February's two orders")
    code, out, _ = run("report", "--db", db("d.sqlite"), "--category", "games", "--category", "STATIONERY")
    H.check("· 2 orders · revenue 43.00" in out, f"games and stationery: {out!r}")


t.scenario("c44_report_prints_markdown_for_a_period_and_categori", case_44)


def case_45():
    run("import", GOOD, "--db", db("e.sqlite"))
    target = os.path.join(TMP, "report.md")
    code, out, _ = run("report", "--db", db("e.sqlite"), "--out", target)
    H.expect((code, out), (0, f"wrote {target}\n"), "exit code and stdout")
    with open(target, encoding="utf-8") as f:
        H.check(f.read().startswith("# Sales report\n"), "the file holds the report")


t.scenario("c45_report_out_writes_the_file", case_45)


def case_46():
    for argv in ([], ["report"], ["report", "--db", "x", "--from", "yesterday"], ["export", "--db", "x"]):
        code, _, err = run(*argv)
        H.expect(code, 2, f"exit code for {argv}")
        H.check("usage:" in err, f"argparse prints the usage for {argv}")


t.scenario("c46_usage_errors_exit_with_2", case_46)


def case_47():
    run("import", GOOD, "--db", db("f.sqlite"))
    p = subprocess.run(
        [sys.executable, "-X", "utf8", "-B", os.path.join(DIR, "cli.py"), "report", "--db", db("f.sqlite")],
        capture_output=True,
        text=True,
        encoding="utf-8",
        timeout=30,
    )
    H.expect(p.returncode, 0, f"exit code (stderr: {p.stderr!r})")
    H.check("· 2 orders ·" in p.stdout, f"the report on stdout: {p.stdout[:80]!r}")


t.scenario("c47_python_cli_py_runs_as_a_script", case_47)
t.finish()
