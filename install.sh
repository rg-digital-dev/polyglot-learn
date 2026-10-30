#!/bin/sh
# Install the `learn` CLI and the polyglot-learn course (Linux and macOS):
#
#   curl -fsSL https://raw.githubusercontent.com/rg-digital-dev/polyglot-learn/main/install.sh | sh
#
# It downloads the latest release's binary for this machine, checks it against
# SHA256SUMS, installs it, and fetches the matching version of the course.
# Running it again updates both; your work in the course's workspace/ stays.
#
# Settings (environment variables):
#   LEARN_BIN_DIR     where `learn` goes          (default: ~/.local/bin)
#   LEARN_COURSE_DIR  where the course goes       (default: ~/polyglot-learn)
#   LEARN_VERSION     a release such as 0.1.0     (default: the latest)

set -eu

REPO="rg-digital-dev/polyglot-learn"
BIN_DIR="${LEARN_BIN_DIR:-$HOME/.local/bin}"
COURSE_DIR="${LEARN_COURSE_DIR:-$HOME/polyglot-learn}"

say() { printf '%s\n' "$*"; }
die() { printf 'install.sh: %s\n' "$*" >&2; exit 1; }
has() { command -v "$1" >/dev/null 2>&1; }

has curl || die "needs curl"
has tar || die "needs tar"

case "$(uname -s)" in
  Linux) os="unknown-linux-musl" ;;
  Darwin) os="apple-darwin" ;;
  *) die "unsupported system $(uname -s). On Windows, use install.ps1 (see the README)." ;;
esac
case "$(uname -m)" in
  x86_64 | amd64) arch="x86_64" ;;
  arm64 | aarch64) arch="aarch64" ;;
  *) die "unsupported CPU $(uname -m). Build from source instead: see the README." ;;
esac
target="$arch-$os"

if [ -n "${LEARN_VERSION:-}" ]; then
  tag="v${LEARN_VERSION#v}"
else
  # /releases/latest redirects to /releases/tag/<tag>.
  url=$(curl -fsSLI -o /dev/null -w '%{url_effective}' "https://github.com/$REPO/releases/latest")
  tag="${url##*/}"
fi
case "$tag" in v[0-9]*) ;; *) die "couldn't find the latest release" ;; esac
version="${tag#v}"
name="learn-$version-$target"
base="https://github.com/$REPO/releases/download/$tag"

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT INT TERM

say "Downloading learn $version for $target..."
curl -fsSL -o "$tmp/$name.tar.gz" "$base/$name.tar.gz" || die "couldn't download $name.tar.gz"
curl -fsSL -o "$tmp/SHA256SUMS" "$base/SHA256SUMS" || die "couldn't download SHA256SUMS"
expected=$(grep " $name.tar.gz\$" "$tmp/SHA256SUMS" | cut -d' ' -f1)
if has sha256sum; then
  actual=$(sha256sum "$tmp/$name.tar.gz" | cut -d' ' -f1)
else
  actual=$(shasum -a 256 "$tmp/$name.tar.gz" | cut -d' ' -f1)
fi
if [ -z "$expected" ] || [ "$expected" != "$actual" ]; then
  die "checksum mismatch for $name.tar.gz; nothing was installed"
fi

tar -xzf "$tmp/$name.tar.gz" -C "$tmp"
mkdir -p "$BIN_DIR"
cp "$tmp/$name/learn" "$BIN_DIR/learn.new"
chmod 755 "$BIN_DIR/learn.new"
mv -f "$BIN_DIR/learn.new" "$BIN_DIR/learn"
if [ "$os" = "apple-darwin" ] && has xattr; then
  # The binary isn't signed: clear the download flag so macOS lets it run.
  xattr -d com.apple.quarantine "$BIN_DIR/learn" 2>/dev/null || true
fi
say "Installed $BIN_DIR/learn"

# The course, at the same version as the binary.
if [ -f "$COURSE_DIR/course.toml" ]; then
  if [ -d "$COURSE_DIR/.git" ] && has git; then
    say "Updating the course in $COURSE_DIR to $tag..."
    if git -C "$COURSE_DIR" fetch -q --depth 1 origin tag "$tag" && git -C "$COURSE_DIR" -c advice.detachedHead=false checkout -q "$tag"; then
      :
    else
      say "  (couldn't update it; check for local changes with git -C \"$COURSE_DIR\" status)"
    fi
  else
    say "Keeping the course in $COURSE_DIR as it is."
  fi
elif [ -e "$COURSE_DIR" ] && [ -n "$(ls -A "$COURSE_DIR" 2>/dev/null)" ]; then
  die "$COURSE_DIR exists but isn't the course. Set LEARN_COURSE_DIR to another folder."
elif has git; then
  say "Fetching the course into $COURSE_DIR..."
  # init + fetch the tag (rather than clone --branch): quiet for annotated tags,
  # and the same steps a later update uses.
  git init -q "$COURSE_DIR"
  git -C "$COURSE_DIR" remote add origin "https://github.com/$REPO"
  git -C "$COURSE_DIR" fetch -q --depth 1 origin tag "$tag" || die "couldn't fetch the course"
  git -C "$COURSE_DIR" -c advice.detachedHead=false checkout -q "$tag"
else
  say "Fetching the course into $COURSE_DIR (no git found, so as a plain folder)..."
  curl -fsSL -o "$tmp/course.tar.gz" "https://github.com/$REPO/archive/refs/tags/$tag.tar.gz"
  mkdir -p "$COURSE_DIR"
  tar -xzf "$tmp/course.tar.gz" -C "$COURSE_DIR" --strip-components 1
fi

say ""
say "Checking which language toolchains you have:"
(cd "$COURSE_DIR" && "$BIN_DIR/learn" doctor) || true

say ""
case ":$PATH:" in
  *":$BIN_DIR:"*) ;;
  *)
    say "$BIN_DIR isn't on your PATH yet. Add this line to your ~/.bashrc or ~/.zshrc,"
    say "then open a new terminal:"
    say "  export PATH=\"$BIN_DIR:\$PATH\""
    say ""
    ;;
esac
say "Then start learning:"
say "  cd $COURSE_DIR"
say "  learn init           # which language do you already know?"
say "  learn start rust     # or typescript, go, php, lua, qml"
say "  learn watch          # or learn serve for the web UI"
