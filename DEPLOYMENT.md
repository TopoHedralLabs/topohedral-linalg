# Deploying Documentation to GitHub Pages

The project has two separate documentation sites, both deployed from this repo to GitHub Pages via a single workflow:

- **High-level docs** (mkdocs-material): `https://topohedrallabs.github.io/topohedral-linalg/`
- **API reference** (cargo doc): `https://topohedrallabs.github.io/topohedral-linalg/api/topohedral_linalg/`

---

## One-time setup

### 1. Enable GitHub Pages

1. Go to **Settings → Pages** in the GitHub repo.
2. Under **Build and deployment → Source**, select **GitHub Actions**.
3. Click **Save**.

### 2. Update `site_url` in mkdocs.yml

Set the correct base URL so mkdocs generates correct canonical links and the sitemap:

```yaml
site_url: https://topohedrallabs.github.io/topohedral-linalg/
```

### 3. Add the workflow file

Create `.github/workflows/docs.yml`:

```yaml
name: Deploy Docs
on:
  push:
    branches: [main]

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  deploy:
    runs-on: ubuntu-latest
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - uses: actions/checkout@v4

      - name: Install OpenBLAS
        run: sudo apt-get install -y libopenblas-dev

      - uses: dtolnay/rust-toolchain@nightly

      - name: Build cargo doc
        run: cargo doc --no-deps

      - uses: actions/setup-python@v5
        with:
          python-version: '3.x'

      - name: Build mkdocs site
        run: |
          pip install -r docs/website/requirements.txt
          cd docs/website && mkdocs build

      - name: Assemble deploy directory
        run: |
          mkdir -p deploy/api
          cp -r docs/website/site/. deploy/
          cp -r target/doc/. deploy/api/

      - uses: actions/upload-pages-artifact@v3
        with:
          path: ./deploy

      - id: deployment
        uses: actions/deploy-pages@v4
```

---

## Cross-linking between the two sites

### From mkdocs → API docs

Add a link anywhere in the mkdocs source (e.g., `docs/getting-started.md`):

```markdown
See the [API Reference](https://topohedrallabs.github.io/topohedral-linalg/api/topohedral_linalg/).
```

### From cargo doc → mkdocs site

Add a top-level doc comment in `src/lib.rs` linking back:

```rust
//! Full documentation and user guide: <https://topohedrallabs.github.io/topohedral-linalg/>
```

---

## Previewing locally

Because the two sites are assembled post-build, use a static file server rather than `mkdocs serve`:

```sh
bash docs/website/build.sh
python3 -m http.server 8000 --directory docs/website/site
```

Then open <http://127.0.0.1:8000> for the mkdocs site and <http://127.0.0.1:8000/api/topohedral_linalg/> for the API docs.
