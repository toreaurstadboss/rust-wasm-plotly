# StackBlitz ready Plotly 2D scatter demo

This project now supports two flows. The Vite app renders a 2D Plotly scatter chart with a shuffle interaction, and Rust also has a native `main.rs` that generates a standalone HTML preview. Vite handles the local dev server and module loading, while Rust generates the plot specification that the browser loader passes to Plotly.

You can run the app directly in StackBlitz from [this GitHub import link](https://stackblitz.com/~/github.com/toreaurstadboss/rust-wasm-plotly). StackBlitz uses the same Vite entry point, so the chart and shuffle behavior match the local setup.

The repository still includes the Rust/WASM source tree and generated `pkg/` output, and the browser demo keeps almost all plot logic in Rust. The JavaScript entry point is a thin loader that initializes wasm and renders the JSON spec. If you want the native Rust flow, run `cargo run` from the `rust/` folder and it will generate `2dscatter.html` in the workspace root.

## Run locally

1. Install dependencies.

```bash
npm install
```

2. Start the development server.

```bash
npm run dev
```

3. Open the URL shown by Vite, usually `http://localhost:5173`.

## Optional wasm rebuild

If you want to regenerate the Rust/WASM output in `pkg/`, run:

```bash
npm run build:wasm
```

## Native Rust preview

If you want the plain Rust entry point instead of the Vite app, run:

```bash
cd rust
cargo run
```

That writes `2dscatter.html` at the workspace root and opens it on Windows.
