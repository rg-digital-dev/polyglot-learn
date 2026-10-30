# QML track

This track teaches the JavaScript side of **QML (Qt 6)**: functions with type
annotations inside a QML object. Each solution is a small component whose root
is a `QtObject`, and the tests are Qt Quick Test `TestCase` files run by
`qmltestrunner`. No window is opened; tests run on the offscreen platform.

## What you need

Qt 6 with Qt Declarative and `qmltestrunner`; 6.7 or newer is recommended
(type annotations are enforced on every call from 6.7 on). `learn` looks on
your `PATH` and in the usual Qt install folders.

## Install

- **Windows:** `pip install aqtinstall`, then
  `aqt install-qt windows desktop 6.10.3 win64_msvc2022_64 -O C:\Qt`.
  aqtinstall 3.3.0 cannot install Qt 6.11 or newer because Qt changed its
  repository layout. For a newer Qt, use the
  [Qt Online Installer](https://www.qt.io/download-qt-installer-oss) (free Qt
  account; choose Qt 6.x > MSVC 2022 64-bit).
- **macOS:** `brew install qt` (or just `brew install qtdeclarative`, which is
  the part this track needs).
- **Arch:** `sudo pacman -S qt6-declarative`. `qmltestrunner` is in `/usr/lib/qt6/bin`.
- **Debian / Ubuntu:** `sudo apt install qt6-declarative-dev-tools qml6-module-qttest qml6-module-qtquick qml6-module-qtqml-workerscript`.
  `qmltestrunner` is in `/usr/lib/qt6/bin` (`/usr/bin/qmltestrunner` belongs to Qt 5's qtchooser).
  Ubuntu 24.04 ships Qt 6.4; for a newer one, `pip install aqtinstall`, then
  `aqt install-qt linux desktop 6.10.3 linux_gcc_64 -O ~/Qt` and add
  `~/Qt/6.10.3/gcc_64/bin` to your `PATH`.
- **Fedora:** `sudo dnf install qt6-qtdeclarative-devel`. It installs `qmltestrunner-qt6`
  in `/usr/bin`.

## How the tests run

`learn` runs, from `workspace/qml`:

```sh
qmltestrunner -platform offscreen -input <slug>    # e.g. -input leap-year
```

You can run the same command yourself. Your code goes in
`<slug>/<PascalSlug>.qml` (for example `leap-year/LeapYear.qml`), and the test
is `<slug>/tst_<slug_with_underscores>.qml`. The test instantiates your
component and calls its functions, checking results with `compare()` and
`verify()`. Fallible functions must `throw new Error(...)`; the stub's own
`TODO: implement` error does not count.

## The JavaScript you get

- ECMAScript 7 per the Qt docs, running in Qt's V4 engine; `let`, `const`,
  arrow functions and template literals work.
- No DOM, no `window`, no Node APIs. `console.log` and `Qt.*` helpers exist.
- Writing to an undeclared variable throws; QML's global object is read-only.
- Type annotations (`int`, `bool`, `string`, `var`) coerce values at call time.
  `int` is 32-bit.
- Gaps that matter in Module 2 (checked on Qt 6.10): `sort` is not stable, and
  `async`/`await`, object spread and helpers such as `flatMap` are missing.

## Editor setup

- **qmlls**, the QML language server, ships with Qt: `<Qt>/bin/qmlls` for
  installer/aqt installs, `qt6-declarative-dev-tools` on Debian. Any
  LSP-capable editor can use it for completion, linting and formatting.

## Where QML is used

- **Quickshell**: a toolkit for building Wayland/X11 desktop shells, bars and
  widgets in QML. Its configs use the same functions, bindings and arrays.
- **KDE Plasma**: Plasma widgets (plasmoids) are written in QML, as are many
  Qt Quick apps on desktop and embedded devices.

## References

- [Qt Quick Test](https://doc.qt.io/qt-6/qtquicktest-index.html)
- [JavaScript Host Environment](https://doc.qt.io/qt-6/qtqml-javascript-hostenvironment.html)
- [QML Language Server](https://doc.qt.io/qt-6/qtqml-tooling-qmlls.html)
- [Quickshell: QML Language guide](https://quickshell.org/docs/v0.3.2/guide/qml-language/)
- [KDE Plasma widget tutorial](https://develop.kde.org/docs/plasma/widget/)
- [aqtinstall manual](https://aqtinstall.readthedocs.io/en/latest/)
