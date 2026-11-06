# Python track

This track targets **Python 3.12 or newer** (CPython, the standard interpreter from python.org). Each exercise is a plain module file; the tests use a tiny harness that `learn` generates, so there is nothing else to install: no pip, no pytest, no virtual environment.

## What you need

A `python3` (or `python`) of version 3.12 or later. Check with `python3 --version` (on Windows: `python --version`). `learn` warns about older versions: the stubs use `list[int]`-style type hints and the notes use `match` and other 3.10+ features. On Windows, a `python3` that opens the Microsoft Store is only an alias; `learn` skips it and looks for a real install.

## Install

- **Windows:** `winget install --id Python.Python.3.14 -e`, then open a new terminal.
- **macOS:** `brew install python`
- **Arch:** `sudo pacman -S python`
- **Debian / Ubuntu:** `sudo apt install python3` (Ubuntu 24.04 ships 3.12, Debian 13 ships 3.13).
- **Fedora:** `sudo dnf install python3`

## How the tests run

`learn` runs, from `workspace/python`:

```sh
python -X utf8 -B <slug>/test_<module>.py      # e.g. python -X utf8 -B leap-year/test_leap_year.py
```

You can run the same command yourself. Your code goes in `<slug>/<module>.py`, where the module name is the slug with underscores. The test file imports it as `M`, and the shared `harness.py` calls each function and compares the result. Lists, dicts and dataclasses compare by value; `True` doesn't count as `1`, and a tuple doesn't count as a list. For fallible functions, raise an exception with a message (usually `ValueError`); returning `None` fails the test.

Type hints are documentation here: Python doesn't check them when it runs. An editor with a type checker (below) does.

## Python details that matter here

- Indentation is the block structure; four spaces per level.
- `int` has no size limit, so a string of twenty digits converts without overflowing; check ranges yourself.
- Lists and dicts are passed by reference: changing a parameter changes the caller's value. Exercises that say "don't modify the input" test that.
- Two exercises have a helper module named like a standard-library module (`numbers` in error-context, `resource` in close-everything). Inside those exercise folders, `import numbers` gets the local file; that's intended.

## Editor setup

- Any editor with a Python language server works: VS Code with the Python extension (Pylance), PyCharm, Zed, or Neovim with `basedpyright` or `pyright`.
- Point it at the same interpreter `learn` uses (`learn doctor` shows its path).

## References

- [The Python Tutorial](https://docs.python.org/3/tutorial/)
- [The Python Standard Library](https://docs.python.org/3/library/)
- [What's New in Python](https://docs.python.org/3/whatsnew/)
- [Exercism Python track](https://exercism.org/tracks/python)
