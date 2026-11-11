# PHP track

This track uses the plain `php` command-line interpreter. There is no Composer, no PHPUnit and no web server: each exercise is one PHP file with a global function, and its test file loads a tiny harness that `learn` generates.

## What you need

**PHP 8.1 or newer** (CLI). Check with `php --version`. No extensions beyond the defaults are needed; `ctype` and `pcre` are built in.

## Install

- **Windows:** `winget install --id PHP.PHP.8.5 -e`, then open a new terminal. The package runs without a `php.ini`. If you later need an extension, copy `php.ini-development` to `php.ini` in the folder that holds `php.exe` and enable the extension there. `php --ini` shows which file is loaded.
- **macOS:** `brew install php`
- **Arch:** `sudo pacman -S php`
- **Debian / Ubuntu:** `sudo apt install php-cli` (Debian 12 and Ubuntu 22.04 or newer ship 8.1+)
- **Fedora:** `sudo dnf install php-cli`

## How the tests run

`learn` runs, from `workspace/php`:

```sh
php <stage>/<slug>/<slug>.test.php      # e.g. php 01-foundations/hello/hello.test.php
```

You can run the same command yourself. Each exercise sits inside a numbered folder for its stage. The test file starts with `declare(strict_types=1);`, loads your solution and the shared `harness.php`, and runs every case from the exercise spec.

The path ends with a project: a tiny web framework built over eight stages in one folder, `workspace/php/NN-web-framework/`, from readonly requests and responses to a router, middleware, validation with attributes, escaping templates, sessions with CSRF tokens, a JSON API and an autowiring container, with a front controller you can run with `php -S localhost:8000 public/index.php`. Each stage adds a file and its test file (`s1-http.test.php`, …) once the stage before passes; your code carries forward, and `learn` runs every test file so far with `php harness.php NN-web-framework`, each in its own process. `PATH.md` in `workspace/php/` shows where you are. The project needs PHP 8.1 or newer and only the extensions the standard `php-cli` package ships: no mbstring.

Things the tests rely on:

- Your function keeps its name and signature, e.g. `function greet(string $name): string`.
- Lists and maps are plain `array`s. Maps are compared after sorting their keys, so insertion order does not matter.
- Records (from Module 2) are objects of the classes in the stub, compared by class and property values.
- Errors are thrown exceptions. The stub's `RuntimeException('TODO: implement ...')` never counts as a pass.

## Editor setup

- VS Code: install the "PHP Intelephense" extension, which reads your PHPDoc (`@param list<int> $scores`) and flags type mistakes.
- PhpStorm works out of the box.
- `php -l file.php` checks a file's syntax without running it.
- `php -a` opens an interactive shell for quick experiments.

## References

- [PHP manual](https://www.php.net/manual/en/)
- [PHP manual: Installation on Windows](https://www.php.net/manual/en/install.windows.php)
- [Exercism PHP track](https://exercism.org/tracks/php)
