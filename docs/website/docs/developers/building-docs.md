# Building Docs Locally

The `build-docs.sh` script at the repo root lets you build and preview the full documentation site locally — including versioned MkDocs pages and the Rust API docs — without pushing anything to GitHub.

## Usage

```bash
./build-docs.sh              # uses version label "local-test"
./build-docs.sh v1.2.0       # test with a specific version label
```

Once running, the site is served at `http://localhost:8000`. Press Ctrl+C to stop; the script cleans up after itself automatically.

## How it works

### Shebang and safety options

```bash
#!/usr/bin/env bash
set -euo pipefail
```

The shebang (`#!`) tells the OS to run this file with `bash`. `/usr/bin/env bash` finds `bash` on your PATH rather than hardcoding a location, which is more portable.

`set -euo pipefail` enables three safety options:

- `-e` — exit immediately if any command fails
- `-u` — treat undefined variables as errors
- `-o pipefail` — if any command in a pipeline (e.g. `foo | bar`) fails, treat the whole pipeline as failed

### Variables

```bash
VERSION="${1:-local-test}"
TEST_BRANCH="docs-local-build"
WORKTREE_PATH="/tmp/docs-local-build"
```

`$1` is the first argument passed to the script. The `:-local-test` part is a default: if no argument is given, `VERSION` becomes `local-test`. The remaining two are named constants so the same values aren't repeated throughout the script.

### Finding the repo root

```bash
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
```

This finds the directory containing the script itself, regardless of where you run it from:

- `${BASH_SOURCE[0]}` — path to this script file
- `dirname ...` — strips the filename, leaving just the directory
- `cd ... && pwd` — changes into that directory and prints its absolute path
- `$(...)` — runs a command and captures its output as a string

So if the script lives at `/home/you/project/build-docs.sh`, `REPO_ROOT` becomes `/home/you/project`.

### Cleanup on exit

```bash
cleanup() {
    git -C "$REPO_ROOT" worktree remove --force "$WORKTREE_PATH" 2>/dev/null || true
    git -C "$REPO_ROOT" branch -D "$TEST_BRANCH" 2>/dev/null || true
}
trap cleanup EXIT
```

`cleanup` removes the temporary git worktree and branch created during the script. `2>/dev/null` redirects error output to nowhere (silencing errors if the branch doesn't exist), and `|| true` means "if this command fails, that's fine" — preventing `-e` from aborting during cleanup.

`trap cleanup EXIT` registers `cleanup` to run automatically whenever the script exits — whether normally, via Ctrl+C, or due to an error.

### Building cargo doc

```bash
RUSTDOCFLAGS="--html-in-header $(pwd)/docs/rustdoc-html/custom-header.html" cargo doc --no-deps
```

Setting an environment variable (`RUSTDOCFLAGS`) on the same line as a command passes it only to that command, without affecting the rest of the script. `$(pwd)` expands to the current directory at the point of execution.

### Deploying with mike

```bash
mike deploy --branch "$TEST_BRANCH" --update-aliases "$VERSION" latest
mike set-default --branch "$TEST_BRANCH" latest
```

Mike builds the MkDocs site and commits it to the local `docs-local-build` branch (not `gh-pages`, so nothing is touched on the real site). `--update-aliases` makes `latest` point to the deployed version. `set-default` makes the root URL redirect to `latest`.

### Extracting the crate name

```bash
CRATE_NAME=$(grep '^name' "$REPO_ROOT/Cargo.toml" | head -1 | sed 's/.*= *//' | tr -d '"' | tr '-' '_')
```

This is a pipeline — each `|` passes the output of the left command as input to the right:

- `grep '^name'` — finds lines starting with `name`
- `head -1` — keeps only the first match
- `sed 's/.*= *//'` — strips everything up to and including ` = `, leaving just the value
- `tr -d '"'` — deletes all `"` characters
- `tr '-' '_'` — replaces hyphens with underscores (rustdoc uses underscores even if `Cargo.toml` uses hyphens)

### Injecting API docs via a git worktree

```bash
git -C "$REPO_ROOT" worktree add "$WORKTREE_PATH" "$TEST_BRANCH"
mkdir -p "$WORKTREE_PATH/$VERSION/api" "$WORKTREE_PATH/latest/api"
cp -r "$REPO_ROOT/target/doc/." "$WORKTREE_PATH/$VERSION/api/"
cp -r "$REPO_ROOT/target/doc/." "$WORKTREE_PATH/latest/api/"
echo "$API_INDEX" > "$WORKTREE_PATH/$VERSION/api/index.html"
echo "$API_INDEX" > "$WORKTREE_PATH/latest/api/index.html"
```

`git -C <path>` tells git to run as if started in that directory. `worktree add` checks out a branch into a separate directory without switching away from your current branch — so you can write files into `docs-local-build` at `/tmp/docs-local-build` while staying on your normal branch.

`mkdir -p` creates directories including any missing parents, without erroring if they already exist.

`cp -r` copies recursively. The `.` at the end of the source path (`target/doc/.`) copies the *contents* of the directory rather than the directory itself, so you get `api/topohedral_linalg/` rather than `api/doc/topohedral_linalg/`.

`>` writes stdout to a file, overwriting it if it exists. This creates the redirect `index.html` at the root of each `api/` directory — necessary because rustdoc doesn't generate one itself; the actual docs live under `topohedral_linalg/`.

### Committing and serving

```bash
git -C "$WORKTREE_PATH" add .
git -C "$WORKTREE_PATH" commit -m "Local: inject API docs"
git -C "$REPO_ROOT" worktree remove "$WORKTREE_PATH"

mike serve --branch "$TEST_BRANCH"
```

`git add .` stages all changes in the worktree. After committing, the worktree is removed (the branch and its content remain). `mike serve` then reads from the branch and serves the assembled site at `http://localhost:8000`.
