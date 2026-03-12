# 🦀 rust_portfolio

A personal portfolio site built with **Rust**, **Leptos 0.8**, and **WebAssembly** — compiled to a static WASM SPA and served via Trunk. No JavaScript frameworks. No Node runtime. Just Rust all the way down.

> Live at: [langat-moimaritim.vercel.app](https://langat-moimaritim.vercel.app)

---

## Why Rust?

The [original portfolio](https://langat-moi.vercel.app) was built in Next.js. It worked, but it didn't *say* anything about the direction I'm headed. Rust is where I'm going all-in — Leptos, Axum, WASM. Rebuilding the portfolio itself in Rust makes the statement: not just learning it, shipping with it.

---

## Stack

| Layer | Technology |
|---|---|
| UI Framework | [Leptos 0.8](https://leptos.dev) (CSR/WASM) |
| Routing | `leptos_router` |
| Meta | `leptos_meta` |
| Build Tool | [Trunk](https://trunkrs.dev) |
| Styling | Vanilla CSS (JetBrains Mono + Syne) |
| Contact Form | [Formspree](https://formspree.io) via `web-sys` fetch |
| Target | `wasm32-unknown-unknown` |
| Deploy | Vercel (static) |

---

## Project Structure

```
rust_portfolio/
├── src/
│   ├── lib.rs                  # WASM entry point
│   ├── components/
│   │   ├── mod.rs
│   │   ├── nav.rs              # Sticky nav with mobile hamburger
│   │   ├── terminal.rs         # Animated terminal hero widget
│   │   ├── project_card.rs     # Reusable project card component
│   │   └── skills.rs           # Skill bars with category colour coding
│   ├── pages/
│   │   ├── mod.rs
│   │   ├── home.rs             # Hero + About + Featured Work + Skills
│   │   ├── projects.rs         # All projects with live filter bar
│   │   ├── iot.rs              # IoT focus areas + projects + certification
│   │   ├── contact.rs          # Contact form (Formspree) + social links
│   │   └── not_found.rs        # 404 styled as Rust compiler error E0404
│   └── data/mod.rs             # All static project + skill data
├── main.css                # Dark terminal theme, fully custom
├── public/
│   └── images/                 # Project screenshots
├── index.html                  # Trunk entry point
├── Trunk.toml                  # Trunk build config
└── Cargo.toml
```

---

## Prerequisites

- [Rust](https://rustup.rs) stable, **1.82 or newer** (see challenge #2 below)
- `wasm32-unknown-unknown` target
- [Trunk](https://trunkrs.dev)

```bash
# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk
```

---

## Development

```bash
# Clone
git clone https://github.com/M0imaritim/rust_portfolio
cd rust_portfolio

# Serve with hot reload at http://localhost:3000
trunk serve
```

Trunk reads `index.html`, compiles the `hydrate` feature, and serves the WASM bundle with live reload.

---

## Production Build

```bash
trunk build --release
```

Output lands in `dist/`. Deploy the entire `dist/` folder to any static host (Vercel, Netlify, GitHub Pages, etc.).

For Vercel, add a `vercel.json` to handle SPA routing:

```json
{
  "rewrites": [{ "source": "/(.*)", "destination": "/index.html" }]
}
```

---

## Pages

| Route | Description |
|---|---|
| `/` | Hero terminal, about section, featured projects, skill bars |
| `/projects` | All projects with filter bar (All / Live / In Progress / 🦀 Rust) |
| `/iot` | IoT focus areas, smart home project, IoT certification |
| `/contact` | Contact form + GitHub / LinkedIn / availability status |
| `*` | 404 styled as Rust compiler error `E0404` |

---

## Design

Dark terminal aesthetic throughout:

- **Font**: JetBrains Mono (code/body) + Syne (headings)
- **Palette**: `#0a0a0a` background · `#e05a2b` rust orange · `#3fb950` green · `#58a6ff` blue
- **Details**: Scanline texture overlay, blinking cursor animations, skill bars with per-category colours, responsive hamburger nav at 640px

---

## Contact Form

The contact form submits to [Formspree](https://formspree.io) using a raw `web-sys` fetch. All browser API calls are scoped inside a `#[cfg(feature = "hydrate")]` block.

To wire it to your own endpoint, replace the URL in `contact.rs`:

```rust
"https://formspree.io/f/YOUR_FORM_ID"
```

Create a free account at formspree.io, create a new form, and you'll get a URL like `https://formspree.io/f/xpwzgkla` — that last segment is your ID.

---

---

# Build Challenges & Learning Points

This section documents every significant error encountered during the build, the root cause of each one, and what the fix revealed about Leptos and the Rust WASM toolchain. It exists so that future work on this project (and similar projects) doesn't repeat the same debugging loops.

---

## Challenge 1 — Wrong Build Tool: `cargo-leptos` vs `trunk`

**What happened:** The project was initially set up with `cargo-leptos` in mind — a `Leptos.toml` was generated, a `[[bin]]` section was added to `Cargo.toml`, and full SSR deps (`axum`, `tokio`, `tower`, `leptos_axum`) were wired up. After running `cargo install cargo-leptos`, it failed because the system's Rust was 1.75 and `cargo-leptos 0.3.x` requires 1.82+. Even falling back to `cargo-leptos 0.2.28` failed because a transitive dep required `edition2024`, also unsupported.

**Root cause:** The wrong tool was chosen for the job. A static portfolio has no server-side logic, no dynamic routes, no auth — SSR via Axum is pure overhead.

**Fix:** Drop `cargo-leptos`, `Leptos.toml`, `src/main.rs`, all SSR deps, and the `[[bin]]` section. Use `trunk` only. Switch to a WASM SPA.

**Learning point:** `cargo-leptos` is for SSR apps where Rust runs on the server and hydrates on the client. `trunk` is for WASM SPAs where all Rust compiles to the browser. A portfolio is static content — trunk is the right tool, and the output (static files) deploys identically to Vercel or Netlify.

---

## Challenge 2 — `edition = "2024"` Breaks Transitive Dependencies

**Error:**
```
feature `edition2024` is required
The package requires the Cargo feature called `edition2024`, but that feature
is not stabilized in this version of Cargo (1.75.0).
```

**What happened:** `Cargo.toml` was initialised with `edition = "2024"`. A transitive dependency (`base64ct`) declared its own manifest using `edition2024`, which isn't recognized by older Cargo versions.

**Fix:** Change to `edition = "2021"`.

**Learning point:** Rust edition in `Cargo.toml` affects how the compiler parses your own code, but it also cascades. If any dependency in the tree uses `edition2024` and your toolchain is `< 1.85`, the entire build fails. Don't use `edition = "2024"` unless you're confirmed on Rust 1.85+. Use `rustc --version` to check before starting a project.

---

## Challenge 3 — Feature Definition Syntax Error

**Error:**
```
error: invalid type: sequence, expected a version string like "0.9.8"
--> Cargo.toml:30:11
```

**What happened:** The `hydrate` feature was written as a single long line:
```toml
hydrate = ["leptos/hydrate", "dep:wasm-bindgen", "dep:web-sys", "dep:js-sys", "dep:wasm-bindgen-futures"]
```

**Fix:** Write feature arrays in multiline format:
```toml
hydrate = [
    "leptos/hydrate",
    "dep:wasm-bindgen",
    "dep:wasm-bindgen-futures",
    "dep:js-sys",
    "dep:web-sys",
]
```

**Learning point:** TOML parsers can misinterpret long inline arrays in certain positions within a manifest. The multiline form is unambiguous and is the preferred style for anything with more than 2–3 entries.

---

## Challenge 4 — `dep:` Prefix Requires `optional = true`

**Error:**
```
feature `hydrate` includes `dep:wasm-bindgen`, but `wasm-bindgen` is not an optional dependency
A non-optional dependency of the same name is defined; consider adding `optional = true`
```

**What happened:** `wasm-bindgen`, `wasm-bindgen-futures`, and `web-sys` were declared as regular dependencies in `[dependencies]` (no `optional = true`), but referenced with the `dep:` prefix in the `[features]` block.

**Fix:** Every dep referenced with `dep:x` in a feature **must** have `optional = true` in its declaration:
```toml
wasm-bindgen = { version = "0.2", optional = true }
wasm-bindgen-futures = { version = "0.4", optional = true }
js-sys = { version = "0.3", optional = true }
web-sys = { version = "0.3", optional = true, features = [...] }
```

**Learning point:** The `dep:` prefix in features is the modern Cargo syntax for enabling an optional dependency. Without `optional = true`, the dep is always compiled in — referencing it via `dep:` is a contradiction and Cargo rejects it. This error appeared across multiple iterations because the fix was applied to some deps but not all of them at once.

---

## Challenge 5 — Dev Dependencies Cannot Be Optional

**Error:**
```
dev-dependencies are not allowed to be optional: `wasm-bindgen`
```

**What happened:** During an iteration of fixes, `wasm-bindgen` ended up under `[dev-dependencies]` with `optional = true`. Cargo forbids this — dev deps apply only to test and bench targets and cannot be feature-gated.

**Fix:** Move all WASM-related deps to `[dependencies]`. Remove `wasm-bindgen-test` and any other testing tools from `[dev-dependencies]` if they aren't actively in use.

**Learning point:** `optional = true` is only valid under `[dependencies]`. The `[dev-dependencies]` section is always fully included for test builds. It has no concept of being conditionally enabled.

---

## Challenge 6 — `[[bin]]` Section Conflicts with Trunk

**Error:**
```
error from build pipeline: more than one target artifact
```

**What happened:** The initial `Cargo.toml` had a `[[bin]]` section pointing to `src/main.rs` — a leftover from the SSR scaffolding phase. Trunk expects to build exactly one artifact: the `cdylib` output of the library crate. When it finds both a lib and a bin, it doesn't know which to use.

**Fix:** Delete `src/main.rs` and remove the `[[bin]]` section from `Cargo.toml`.

**Learning point:** Trunk builds the `cdylib` output of `[lib]` into WASM. It does not build binaries. A `[[bin]]` section is the SSR world, not the trunk world. If migrating from an SSR scaffold, scrub all binary-target artifacts before switching to trunk.

---

## Challenge 7 — `serde::Deserialize` Cannot Handle `&[&str]`

**Error:**
```
the trait bound `&[&str]: serde::Deserialize<'de>` is not satisfied
```

**What happened:** The `Project` struct had `#[derive(Serialize, Deserialize)]` on it, but included a `tags: &'static [&'static str]` field. Serde cannot deserialize into borrowed slice references — it has nowhere to allocate the referenced data.

**Fix:** Remove `Serialize` and `Deserialize` from all data structs entirely. Also remove `serde` and `serde_json` from `Cargo.toml`.

**Learning point:** `Serialize`/`Deserialize` is only needed when passing data across a serialization boundary — e.g., over a network, through a file, or between server and client in SSR. In a WASM SPA, all data lives in browser memory. There is no boundary. All data structs are plain Rust — no serde involved.

---

## Challenge 8 — Leptos `<A>` Component Doesn't Accept `class` Directly

**Error:**
```
error[E0599]: the method `class` exists for struct `APropsBuilder<_>`, but its trait bounds were not satisfied
```

**What happened:** Code like `<A href="/projects" class="btn btn-primary">` was used throughout. Native HTML elements like `<div class="...">` accept `class` directly in Leptos's `view!` macro. Leptos *components* (capitalised, like `<A>`) are Rust structs — they only accept props they explicitly declare. `class` is not a declared prop of `<A>`.

**Fix:** Use the `attr:` prefix for HTML attributes on Leptos components everywhere:
```rust
// Wrong
<A href="/projects" class="btn btn-primary">"View Projects"</A>

// Right
<A href="/projects" attr:class="btn btn-primary">"View Projects"</A>
```

This had to be fixed in every file that used `<A>`: nav, home page, 404, and more.

**Learning point:** In Leptos 0.7+, `attr:name` is how you pass arbitrary HTML attributes to components that don't explicitly declare them as typed props. Native HTML elements accept attributes directly. Leptos components need `attr:`. This is unintuitive at first but consistent once you know the rule.

---

## Challenge 9 — Lifetime Error: Borrowed `project.status` Inside `view!`

**Error:**
```
error[E0597]: `project.status` does not live long enough
...opaque type requires that `project.status` is borrowed for `'static`
```

**What happened:** In `ProjectCard`, `project.status.label()` was called inside the `view!` macro. The `label()` method returns a `&str` borrowed from `project.status`. The Leptos `view!` macro generates reactive closures that must be `'static` — they need to potentially outlive the current stack frame. The borrow from a local `project` binding cannot satisfy that.

**Fix:** Extract an owned `String` before the view macro:
```rust
let status_label = project.status.label().to_string(); // owned, no borrow remaining
```

The other fields (`title`, `description`, `tags`) are `&'static str` — they reference string literals embedded in the binary and live forever, so they are fine without `.to_string()`.

**Learning point:** Leptos reactive closures require `'static` data or owned values. The rule: if you're borrowing from a local binding and intending to use the result inside `view!`, you'll hit this error. The pattern is to resolve all borrows into owned `String`s *before* the `view!` block begins.

---

## Challenge 10 — Top-Level `use wasm_bindgen` Fails Without the Feature

**Error:**
```
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `wasm_bindgen`
--> src/lib.rs:4:5
|
4 | use wasm_bindgen::prelude::wasm_bindgen;
```

**What happened:** `lib.rs` had `use wasm_bindgen::prelude::wasm_bindgen;` as a top-level unconditional import. But `wasm-bindgen` is an optional dep — it doesn't exist unless the `hydrate` feature is enabled. When Rust checks the crate without that feature (e.g., for the `rlib` build pass), the module simply doesn't exist.

**Fix:** Never write a top-level `use wasm_bindgen::...`. Reference it inline on the attribute only, with the entire function gated behind `#[cfg(feature = "hydrate")]`:

```rust
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    leptos::mount::mount_to_body(App);
}
```

The inline path `wasm_bindgen::prelude::wasm_bindgen` does not require a `use` statement.

**Learning point:** Optional deps in Rust are truly absent from the crate graph when their feature is off. A top-level `use` import is unconditional — it resolves at compile time regardless of what feature flags are set on your binary. Every reference to an optional dep must live inside `#[cfg(feature = "...")]`.

---

## Challenge 11 — Deprecated `web-sys` `RequestInit` API

**Warnings:**
```
warning: use of deprecated method `leptos::web_sys::RequestInit::method`: Use `set_method()` instead
warning: use of deprecated method `leptos::web_sys::RequestInit::mode`: Use `set_mode()` instead
warning: use of deprecated method `leptos::web_sys::RequestInit::body`: Use `set_body()` instead
```

**What happened:** The contact form fetch code used the old fluent builder style for `RequestInit`. In newer `web-sys` versions the builder was replaced with explicit setter methods.

**Fix:**
```rust
// Old (deprecated fluent builder)
opts.method("POST");
opts.mode(RequestMode::Cors);
opts.body(Some(&JsValue::from_str(&body)));

// New (setter methods)
opts.set_method("POST");
opts.set_mode(RequestMode::Cors);
opts.set_body(&JsValue::from_str(&body));
```

**Learning point:** `web-sys` mirrors the browser's Web APIs 1:1. When the spec changes or the Rust bindings are updated, method names change. Always check for deprecation warnings on `web-sys` APIs and use the current setter-style API for `RequestInit`.

---

## Challenge 12 — Browser API Imports in `contact.rs` Without Feature Gate

**Errors:**
```
error[E0432]: unresolved import `wasm_bindgen_futures`
error[E0433]: failed to resolve: use of unresolved module `wasm_bindgen`
error[E0433]: failed to resolve: use of undeclared type `JsValue`
error[E0282]: type annotations needed for `Result<_, _>`
```

**What happened:** `contact.rs` had top-level `use` statements for `wasm_bindgen`, `wasm_bindgen_futures`, and `web_sys`. Same root cause as challenge 10 — these are behind the `hydrate` feature and top-level imports don't respect feature flags.

**Fix:** Move all browser API `use` imports inside the `#[cfg(feature = "hydrate")]` block:
```rust
#[cfg(feature = "hydrate")]
{
    use wasm_bindgen::JsValue;
    use wasm_bindgen_futures::spawn_local;
    use web_sys::{Request, RequestInit, RequestMode, Response};

    spawn_local(async move {
        // fetch logic
    });
}
```

Also, annotate the `resp` binding explicitly so the compiler can infer the `JsFuture` return type:
```rust
let resp: Result<JsValue, JsValue> =
    wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await;
```

And use full path for `dyn_into` to avoid needing an import:
```rust
let response: Response = wasm_bindgen::JsCast::dyn_into(val).unwrap();
```

**Learning point:** The `#[cfg(feature = "hydrate")]` gate must wrap not just the async code body, but also the `use` statements that reference optional deps. A scoped block `{ use ...; code... }` is the cleanest pattern.

---

## Challenge 13 — The Black Screen (Hardest Bug)

**Symptom:** Trunk compiled without errors. The browser loaded the page. No errors in the browser console. The page was completely blank.

**Root cause:** The WASM binary loaded correctly, but the `hydrate` feature was not active at compile time. This meant the `#[wasm_bindgen(start)]` function was never compiled into the binary. The JS bootstrap script initialized the WASM module but found no start function to call, so nothing mounted.

**How it was diagnosed:** Inspecting the generated `dist/index.html` revealed:
```js
import init, * as bindings from '/rust_portfolio-xxxx.js';
const wasm = await init({ module_or_path: '/rust_portfolio-xxxx_bg.wasm' });
window.wasmBindings = bindings;
dispatchEvent(new CustomEvent("TrunkApplicationStarted", {detail: {wasm}}));
```
There was no call to `bindings.main()` or any start function anywhere in the script. If `hydrate` were active, the exported `#[wasm_bindgen(start)]` function would auto-run on `init()`. It didn't.

**Fixes applied:**

1. Add `data-cargo-features="hydrate"` to the `<link data-trunk rel="rust">` tag in `index.html`:
   ```html
   <link data-trunk rel="rust" data-wasm-opt="z" data-cargo-features="hydrate" />
   ```

2. Ensure `Trunk.toml` has proper formatting with a trailing newline. A missing newline caused the file to be silently ignored, meaning `cargo_features = ["hydrate"]` was never read:
   ```toml
   [build]
   cargo_features = ["hydrate"]
   ```

3. Use a top-level `pub` function for the entry point — not nested inside a private `mod` block. The `#[wasm_bindgen(start)]` proc macro is most reliably applied to a top-level public function:
   ```rust
   #[cfg(feature = "hydrate")]
   #[wasm_bindgen::prelude::wasm_bindgen(start)]
   pub fn main() {
       leptos::mount::mount_to_body(App);
   }
   ```

**Learning point:** Trunk has two independent mechanisms for passing Cargo features: the `index.html` link tag (`data-cargo-features`) and `Trunk.toml` (`cargo_features`). If both are absent or either is silently malformed, the feature is never activated and your entry point doesn't exist in the binary. A blank page with no console errors is the signature symptom. The diagnostic is to inspect the generated JS init script in `dist/index.html` — if it doesn't invoke your start function, the hydrate feature wasn't compiled in.

---

## Challenge 14 — Architecture Decision: SSR vs CSR

**The question:** Should this be a Leptos SSR app (Axum backend, `cargo-leptos`) or a WASM SPA (Trunk, CSR only)?

**Initial instinct:** SSR feels more complete — a real Axum backend, actual server rendering, more impressive on a Rust portfolio.

**Reality check:** For a portfolio with purely static content and a third-party contact form, SSR adds zero user-facing benefit. It means maintaining a running Rust server, deploying to a VPS or serverless platform with Rust support, cold start management, and a significantly more complex build pipeline. The original Next.js portfolio was already static — the Rust rebuild should be no different.

**Decision:** CSR/WASM SPA via Trunk. Deploy to Vercel as static files. Every line of the UI is still Rust — it just runs in the browser rather than on a server. The portfolio still showcases Leptos, reactive signals, component architecture, and browser API usage via `web-sys`.

**Learning point:** Choosing the right tool is a skill, not a compromise. SSR is appropriate when you need SEO on dynamic content, server-side auth, database access at render time, or real API routes. For a portfolio, none of those apply. Reaching for SSR by default because it sounds more impressive is architectural cargo-culting.

---

## Challenge 15 — GitHub Actions CI/CD Pipeline

Setting up a working GitHub Actions pipeline for a Trunk + WASM project is non-trivial. GitHub-hosted runners are standard Ubuntu Linux machines — they have Rust installed but none of the WASM-specific tooling. Everything has to be explicitly installed in the workflow, and the order of steps matters.

The first version of the workflow had several mistakes that needed fixing. They are documented as pitfalls below, followed by the corrected final workflow.

### First Draft (with bugs)

```yaml
name: Release to Vercel

on:
  push:
    branches:
      - main

jobs:
  Vercel-Production-Deployment:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3                        # ❌ outdated

      - uses: dtolnay/rust-toolchain@nightly             # ❌ nightly not needed
        with:
          components: clippy, rustfmt

      - name: Setup Rust
        run: |
          rustup target add wasm32-unknown-unknown        # ❌ handled by the action above
          cargo clippy                                    # ❌ checks wrong target
          cargo fmt --check

      - name: Download and install Trunk binary
        run: wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.18.2/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
        # ❌ hardcoded old version, manual download

      - name: Build with Trunk
        run: ./trunk build --features hydrate --release  # ❌ trunk doesn't accept --features

      - name: Deploy to Vercel & Display URL
        working-directory: ./dist
        run: |
          vercel deploy --prod --token=${{ secrets.VERCEL_TOKEN }} >> $GITHUB_STEP_SUMMARY
          # ❌ URL not captured as a step output
```

### The Corrected Workflow

Create `.github/workflows/deploy.yml`:

```yaml
name: Build & Deploy

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  VERCEL_ORG_ID: ${{ secrets.VERCEL_ORG_ID }}
  VERCEL_PROJECT_ID: ${{ secrets.VERCEL_PROJECT_ID }}

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest
    environment: production

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust stable
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
          components: clippy, rustfmt

      - name: Cache Cargo registry and build
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-

      - name: Check formatting
        run: cargo fmt --check

      - name: Clippy (hydrate feature, WASM target)
        run: cargo clippy --target wasm32-unknown-unknown --features hydrate -- -D warnings

      - name: Install Trunk
        uses: jetli/trunk-action@v0.5.0
        with:
          version: latest

      - name: Build
        run: trunk build --release

      - name: Install Vercel CLI
        run: npm install --global vercel@latest

      - name: Pull Vercel environment
        run: vercel pull --yes --environment=production --token=${{ secrets.VERCEL_TOKEN }}

      - name: Deploy to Vercel
        if: github.ref == 'refs/heads/main' && github.event_name == 'push'
        id: deployment
        working-directory: ./dist
        run: |
          url=$(vercel deploy --prod --token=${{ secrets.VERCEL_TOKEN }})
          echo "url=$url" >> $GITHUB_OUTPUT
          echo "### 🚀 Deployed to: $url" >> $GITHUB_STEP_SUMMARY
```

### Pitfall 1 — Using `@nightly` When Stable Works Fine

**What happens:** The workflow uses `dtolnay/rust-toolchain@nightly`. Builds pass intermittently but randomly fail when a nightly regression hits a dependency. CI becomes unreliable with errors that don't reproduce locally.

**Root cause:** There is no reason to use nightly for a standard Leptos project. Nightly was specified by habit — it felt more powerful. But nightly Rust is unstable by definition, and randomly-broken nightly builds waste time debugging CI rather than shipping code.

**Fix:** Use `@stable`:
```yaml
- uses: dtolnay/rust-toolchain@stable
  with:
    targets: wasm32-unknown-unknown
    components: clippy, rustfmt
```

**Learning point:** Only use nightly if your project explicitly requires a nightly-only feature (`#![feature(...)]`). Leptos 0.8 compiles on stable. Using nightly on CI for a stable codebase trades reliability for nothing. When something breaks, it will be a nightly regression, not your code, and it will be confusing to diagnose.

---

### Pitfall 2 — `trunk build --features hydrate` Is Not Valid Syntax

**What happened:** The build step was:
```yaml
- run: ./trunk build --features hydrate --release
```

This fails because `trunk` is not `cargo`. Trunk does not accept `--features` as a CLI flag — that's a Cargo flag. Trunk reads features from `Trunk.toml` or from the `data-cargo-features` attribute in `index.html`.

**Fix:** Just run trunk normally. Features are already configured in `index.html` and `Trunk.toml`:
```yaml
- run: trunk build --release
```

If you want an explicit fallback, pass cargo features through the env var trunk respects:
```yaml
- run: trunk build --release
  env:
    TRUNK_BUILD_CARGO_FEATURES: hydrate
```

**Learning point:** Trunk is a build orchestrator, not a direct cargo wrapper. It calls cargo internally with its own arguments. You configure what goes to cargo via `Trunk.toml` and the `index.html` data attributes — not via trunk's own CLI flags. Confusing trunk flags with cargo flags is easy to do; read `trunk build --help` to see what trunk itself actually accepts.

---

### Pitfall 3 — `actions/checkout@v3` Is Outdated

**What happened:** The workflow used `actions/checkout@v3`. This isn't a breaking issue, but v3 uses Node 16 under the hood which GitHub began deprecating. It generates warnings in the Actions log and will eventually stop working.

**Fix:**
```yaml
- uses: actions/checkout@v4
```

**Learning point:** GitHub Actions have their own versioning independent of your project. Always use the latest major version of official actions (`actions/checkout`, `actions/cache`, `actions/upload-artifact`, etc.). Check https://github.com/actions for the current stable major versions. Pinning to old versions creates silent technical debt.

---

### Pitfall 4 — Hardcoded Old Trunk Version via `wget`

**What happened:** Trunk was installed by manually downloading a specific old release binary:
```yaml
- run: wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.18.2/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
```

This has two problems: `v0.18.2` is several major versions behind (current is `v0.21.x`), and older trunk versions have known compatibility issues with newer Leptos 0.7+ projects. Additionally, the extracted binary only exists in the current directory, which is fragile.

**Fix:** Use the `jetli/trunk-action` which downloads a pre-built binary, adds it to PATH, and supports `version: latest`:
```yaml
- uses: jetli/trunk-action@v0.5.0
  with:
    version: latest
```

**Learning point:** For tools like trunk, wasm-pack, or wasm-bindgen-cli, always check for a dedicated GitHub Action before writing a manual download step. Purpose-built actions handle PATH setup, caching, and version management correctly. Manual `wget | tar` steps are brittle and require manual version maintenance.

---

### Pitfall 5 — Clippy Checking the Wrong Target

**What happened:** The workflow ran:
```yaml
- run: cargo clippy
```

Without specifying a target, clippy checks the native host target (`x86_64-unknown-linux-gnu`). Some WASM-specific code paths are gated behind `#[cfg(feature = "hydrate")]` or `#[cfg(target_arch = "wasm32")]` — clippy on the native target skips those paths entirely. You get a clean clippy pass that misses real issues in your WASM code.

**Fix:** Scope clippy explicitly to the WASM target with the hydrate feature active:
```yaml
- run: cargo clippy --target wasm32-unknown-unknown --features hydrate -- -D warnings
```

**Learning point:** Clippy and the compiler both do target-specific analysis. A clean clippy run on your host target says nothing about your WASM build. For a WASM project, always lint against `wasm32-unknown-unknown`. The `-- -D warnings` flag treats all warnings as errors, which is good CI hygiene — it prevents warning debt from accumulating.

---

### Pitfall 6 — Deployment URL Not Captured

**What happened:** The deploy step was:
```yaml
- run: vercel deploy --prod --token=${{ secrets.VERCEL_TOKEN }} >> $GITHUB_STEP_SUMMARY
```

The `>>` redirection appends the URL to the job summary file, but `vercel deploy` writes status messages to stderr and the URL to stdout. The redirect captures both mixed together, so the summary shows noise alongside the URL. Also the URL is not available as a step output for downstream steps or notifications.

**Fix:** Capture stdout cleanly into a variable first, then write it to both `$GITHUB_OUTPUT` and `$GITHUB_STEP_SUMMARY` separately:
```yaml
- name: Deploy to Vercel
  id: deployment
  working-directory: ./dist
  run: |
    url=$(vercel deploy --prod --token=${{ secrets.VERCEL_TOKEN }})
    echo "url=$url" >> $GITHUB_OUTPUT
    echo "### 🚀 Deployed to: $url" >> $GITHUB_STEP_SUMMARY
```

With `id: deployment` set, downstream steps can reference the URL as `${{ steps.deployment.outputs.url }}`.

**Learning point:** `$GITHUB_STEP_SUMMARY` is for human-readable markdown visible in the Actions UI. `$GITHUB_OUTPUT` is for machine-readable key-value pairs consumed by other steps. They are separate and serve different purposes. When a step produces a meaningful value (like a deployment URL), write it to both.

---

### Pitfall 7 — WASM Target Not Installed on the Runner

**What happens:** The build fails immediately with:

```
error[E0463]: can't find crate for `std`
|
= note: the `wasm32-unknown-unknown` target may not be installed
```

or trunk fails with:

```
error: failed to build WASM files
...could not compile wasm32-unknown-unknown target
```

**Root cause:** GitHub's ubuntu-latest runner ships with Rust stable but only the host target (`x86_64-unknown-linux-gnu`). The `wasm32-unknown-unknown` target is not included by default.

**Fix:** Use the `dtolnay/rust-toolchain` action with an explicit `targets` field:
```yaml
- uses: dtolnay/rust-toolchain@stable
  with:
    targets: wasm32-unknown-unknown
```

This is the canonical Rust toolchain action. It installs stable Rust and adds the WASM target in one step. Do not use the older `actions-rs/toolchain@v1` — it is unmaintained.

**Learning point:** Never assume CI has your target installed. For WASM projects, `wasm32-unknown-unknown` must be explicitly added. The `targets:` key accepts a comma-separated list if you need multiple.

---

### Pitfall 8 — Trunk Not in PATH

**What happens:**
```
/usr/bin/bash: line 1: trunk: command not found
Error: Process completed with exit code 127.
```

**Root cause:** `trunk` is not part of any standard CI image. It must be installed as a workflow step.

**Fix:** Use the `jetli/trunk-action` action, which handles downloading and caching trunk for you:
```yaml
- uses: jetli/trunk-action@v0.5.0
  with:
    version: latest
```

Alternatively, install it via `cargo install` — but this adds 5–10 minutes to every build because it compiles from source:
```yaml
- run: cargo install trunk
```

The `jetli/trunk-action` downloads a pre-built binary and is significantly faster. Use it.

**Learning point:** For any non-standard build tools (trunk, wasm-pack, wasm-bindgen-cli), always check for a dedicated GitHub Action or pre-built binary before falling back to `cargo install`. Compiling tools from source on every CI run adds unnecessary time and burns Actions minutes.

---

### Pitfall 9 — Cargo Build Cache Not Configured

**What happens:** Every run takes 5–15 minutes to compile the full dependency tree from scratch, even when nothing changed.

**Root cause:** By default GitHub Actions does not cache anything between runs. All `target/` output and `~/.cargo/registry` downloads are discarded after each job.

**Fix:** Add an `actions/cache` step before the build, keyed on `Cargo.lock`:
```yaml
- uses: actions/cache@v4
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      target
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    restore-keys: |
      ${{ runner.os }}-cargo-
```

The `key` is a hash of `Cargo.lock` — it invalidates whenever your dep tree changes. The `restore-keys` fallback means you still get a partial cache hit even when the lock file changes, restoring the closest older cache and only rebuilding what changed.

**Learning point:** Rust compile times on CI are painful without caching. The `target/` directory can be several gigabytes for a Leptos project. Caching it keyed on `Cargo.lock` is the single most impactful CI optimisation you can make. Without it, every push waits a long time for a full cold compile.

---

### Pitfall 10 — `hydrate` Feature Not Passed to Trunk on CI

**What happens:** Build succeeds on CI, deployment goes through, but the deployed site shows a blank page — exactly like the local black screen from Challenge 13.

**Root cause:** `trunk build --release` on CI doesn't automatically read `Trunk.toml` if the working directory is wrong, or the `Trunk.toml` doesn't have a trailing newline, or the file was never committed to the repo. The `hydrate` feature isn't activated, so the WASM entry point doesn't exist in the binary.

**Fix:** Either rely on `Trunk.toml` (and make sure it's committed with the correct content and a trailing newline):
```toml
[build]
cargo_features = ["hydrate"]
```

Or pass the feature explicitly in the `run` step as a fallback:
```yaml
- name: Build
  run: trunk build --release --features hydrate
```

Both approaches work. Explicit CLI flags are more visible and easier to debug on CI where you can't inspect intermediate files.

**Learning point:** CI is a clean environment — it has only what's in your repository. `Trunk.toml` must be committed. If it's in `.gitignore` or was never added with `git add`, it doesn't exist on the runner. Always verify that config files your build depends on are tracked in git.

---

### Pitfall 11 — Deploying `dist/` Not the Repo Root

**What happens:** Vercel (or Netlify) deploys the repo root, not the `dist/` output directory. The deployment shows a raw directory listing or a 404 instead of the portfolio.

**Root cause:** Static hosting platforms default to deploying from the repository root. `trunk build` outputs to `dist/` — that's the directory that needs to be served, not the Rust source.

**Fix for Vercel via GitHub Actions:**
```yaml
- uses: amondnet/vercel-action@v25
  with:
    vercel-token: ${{ secrets.VERCEL_TOKEN }}
    vercel-org-id: ${{ secrets.VERCEL_ORG_ID }}
    vercel-project-id: ${{ secrets.VERCEL_PROJECT_ID }}
    working-directory: dist
    vercel-args: '--prod'
```

The `working-directory: dist` tells the action to deploy from the `dist/` folder only.

**Fix for GitHub Pages:**
```yaml
- uses: actions/upload-pages-artifact@v3
  with:
    path: dist

- uses: actions/deploy-pages@v4
```

**Fix for Netlify:**
```yaml
- uses: nwtgck/actions-netlify@v3
  with:
    publish-dir: dist
    production-branch: main
    github-token: ${{ secrets.GITHUB_TOKEN }}
    deploy-message: "Deploy from GitHub Actions"
  env:
    NETLIFY_AUTH_TOKEN: ${{ secrets.NETLIFY_AUTH_TOKEN }}
    NETLIFY_SITE_ID: ${{ secrets.NETLIFY_SITE_ID }}
```

**Learning point:** The build output directory and the deployment source directory are separate concerns. Always specify `dist/` (or whatever trunk's configured output dir is) as the deployment source — never the repo root. Also add `dist/` to `.gitignore` so the built output doesn't get committed back to the repo.

---

### Pitfall 12 — SPA Routing Breaks on Direct URL Access After Deploy

**What happens:** Navigating to `https://yoursite.com/projects` directly returns a 404 from the hosting provider. The home page works but every other route fails on hard refresh or direct link.

**Root cause:** The portfolio is a SPA — there is literally only one HTML file (`dist/index.html`). All routing is handled by `leptos_router` running in the browser. When a user navigates directly to `/projects`, the server looks for `dist/projects/index.html`, which doesn't exist, and returns 404.

**Fix — Vercel:** Add a `vercel.json` at the project root (committed to the repo):
```json
{
  "rewrites": [{ "source": "/(.*)", "destination": "/index.html" }]
}
```

**Fix — Netlify:** Add a `public/_redirects` file (or `netlify.toml`):
```
/*  /index.html  200
```

**Fix — GitHub Pages:** Duplicate `index.html` as `404.html` in the `dist/` directory so GitHub Pages falls back to it on unknown routes. Add to the workflow:
```yaml
- name: Copy index.html to 404.html for SPA routing
  run: cp dist/index.html dist/404.html
```

**Learning point:** Every client-side router (React Router, `leptos_router`, Vue Router, etc.) requires server-side fallback configuration to support direct URL access and page refreshes. The server must return `index.html` for any path it doesn't recognise, then let the JS/WASM router handle the rest. This is not automatic — you must configure it explicitly for each hosting platform.

---

### Secrets Setup for Vercel Deployment

In your GitHub repository, go to **Settings → Secrets and variables → Actions** and add:

| Secret | Where to find it |
|---|---|
| `VERCEL_TOKEN` | Vercel dashboard → Account Settings → Tokens |
| `VERCEL_ORG_ID` | `.vercel/project.json` after running `vercel link` locally |
| `VERCEL_PROJECT_ID` | `.vercel/project.json` after running `vercel link` locally |

Run `vercel link` locally once to generate `.vercel/project.json`, read the IDs from it, then add them to GitHub secrets. Do not commit `.vercel/project.json` — add it to `.gitignore`.

---

## Summary Table

| # | Error / Issue | Root Cause | Fix |
|---|---|---|---|
| 1 | `cargo-leptos` install failure | Wrong tool for the job; toolchain too old | Switch to `trunk`, drop SSR |
| 2 | `edition2024` not stabilized | `edition = "2024"` on Rust < 1.85 | Use `edition = "2021"` |
| 3 | TOML parse error on features | Long single-line feature array | Use multiline array format |
| 4 | `dep:x` but `x` not optional | Missing `optional = true` on deps | Add `optional = true` to all `dep:`-referenced deps |
| 5 | Dev-dep cannot be optional | `optional = true` under `[dev-dependencies]` | Move to `[dependencies]` |
| 6 | "More than one target artifact" | `[[bin]]` section left from SSR scaffold | Remove `[[bin]]` and `src/main.rs` |
| 7 | `&[&str]: Deserialize` not satisfied | `Serialize`/`Deserialize` on structs with borrowed fields | Remove serde derives entirely |
| 8 | `class` not a valid prop on `<A>` | Components need `attr:` prefix for HTML attributes | Use `attr:class` on all Leptos components |
| 9 | `project.status` lifetime error | Borrow from local inside `view!` closure | Extract owned `String` before `view!` |
| 10 | `wasm_bindgen` unresolved in `lib.rs` | Top-level `use` of optional dep | Use inline path + `#[cfg(feature)]` gate |
| 11 | Deprecated `RequestInit` methods | Old `web-sys` fluent builder API | Use `set_method()`, `set_mode()`, `set_body()` |
| 12 | `wasm_bindgen` unresolved in `contact.rs` | Top-level `use` of optional deps | Move `use` statements inside `#[cfg(feature)]` block |
| 13 | Blank page, no console errors | `hydrate` feature not active at compile time | Add `data-cargo-features="hydrate"` to `index.html` |
| 14 | Over-engineered SSR architecture | Defaulting to SSR for a static portfolio | CSR/WASM SPA is the right tool |
| 15a | `@nightly` toolchain used unnecessarily | Nightly breaks randomly; stable suffices for Leptos | Use `dtolnay/rust-toolchain@stable` |
| 15b | `trunk build --features hydrate` fails | Trunk doesn't accept `--features`; that's a cargo flag | Configure features via `Trunk.toml` or `index.html` |
| 15c | `actions/checkout@v3` deprecation warnings | v3 uses deprecated Node 16 runtime | Use `actions/checkout@v4` |
| 15d | Hardcoded old Trunk version via `wget` | Old trunk incompatible with Leptos 0.7+; brittle download | Use `jetli/trunk-action@v0.5.0` with `version: latest` |
| 15e | Clippy passes but misses WASM code paths | Running clippy without `--target wasm32-unknown-unknown` | Use `cargo clippy --target wasm32-unknown-unknown --features hydrate` |
| 15f | Deployment URL garbled in step summary | `vercel deploy` stderr/stdout mixed via `>>` redirect | Capture with `url=$(...)`, write to `$GITHUB_OUTPUT` and `$GITHUB_STEP_SUMMARY` separately |
| 15g | WASM target missing on CI | Runners only have host target by default | Use `dtolnay/rust-toolchain@stable` with `targets: wasm32-unknown-unknown` |
| 15h | Trunk not found on CI | Trunk is not pre-installed on any runner | Use `jetli/trunk-action` for fast pre-built install |
| 15i | CI compile times 10–15 min | No `target/` or registry cache configured | Add `actions/cache` keyed on `Cargo.lock` hash |
| 15j | Blank page after CI deploy | `hydrate` feature not passed to trunk | Commit `Trunk.toml` with `cargo_features = ["hydrate"]` |
| 15k | Wrong directory deployed | Platform defaults to repo root, not `dist/` | Set `working-directory: dist` in the deploy action |
| 15l | Direct URLs return 404 after deploy | SPA routing not configured server-side | Add `vercel.json` rewrites or platform-equivalent fallback |
