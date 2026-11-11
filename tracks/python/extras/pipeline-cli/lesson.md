# A command line with argparse

`argparse` turns `sys.argv` into a namespace of typed values, writes `--help` for you, and prints a usage message for mistakes. Subcommands, like `git commit` and `git push`, are subparsers:

```python
import argparse
from datetime import date

p = argparse.ArgumentParser(prog="notes")
sub = p.add_subparsers(dest="command", required=True)

add = sub.add_parser("add", help="add a note")
add.add_argument("text")
add.add_argument("--tag", action="append", default=[])      # --tag a --tag b

show = sub.add_parser("show", help="list notes")
show.add_argument("--since", type=date.fromisoformat)        # converted and checked

print(p.parse_args(["add", "buy milk", "--tag", "home"]))
# Namespace(command='add', text='buy milk', tag=['home'])
print(p.parse_args(["show", "--since", "2026-01-01"]).since)  # 2026-01-01, a date

try:
    p.parse_args(["show", "--since", "yesterday"])
except SystemExit as e:
    print("exit code", e.code)                                # usage printed on stderr, then 2
```

argparse ends the program on a usage error by raising `SystemExit(2)`. A `main(argv) -> int` that catches it, and returns exit codes instead of calling `sys.exit` itself, can be tested by calling it like any function. The file ends with the usual guard, which runs only for `python cli.py …`, not on `import cli`:

```python
if __name__ == "__main__":
    sys.exit(main())
```

Results go to stdout and problems to stderr (`print(..., file=sys.stderr)`). Then `python cli.py report --db shop.sqlite > report.md` writes only the report to the file. The exit code tells scripts how it went: 0 for success, 1 when something needs attention, 2 for a usage error.

<!-- from:typescript -->
## Coming from TypeScript
- argparse is the standard-library version of commander or yargs: declare the arguments, and get help and errors for free.
- `if __name__ == "__main__":` is Node's `if (require.main === module)`.
- `SystemExit` is an exception, so `sys.exit()` can be caught, unlike `process.exit()`, which ends the process on the spot.
<!-- /from -->

<!-- from:default -->
## Key points
- Subcommands with `add_subparsers(dest=..., required=True)`; `type=` converts and validates.
- Keep `main(argv) -> int` testable: no `sys.exit` inside, only at the bottom of the file.
- Output on stdout, problems on stderr, and the outcome in the exit code.
<!-- /from -->

## Common mistake
Reading `sys.argv` inside `main` instead of taking `argv` as a parameter. Tests then have to patch `sys.argv`, and two tests running different commands can't share a process cleanly. Pass `argv` to `parse_args(argv)`; `None` means "use sys.argv".

## In this stage
Implement `main` in `cli.py`; its docstring lists both subcommands. Then the pipeline is done. Try it on your own data:

```sh
python cli.py import orders.csv web.jsonl --db shop.sqlite
python cli.py report --db shop.sqlite --from 2026-01-01 --out report.md
```
