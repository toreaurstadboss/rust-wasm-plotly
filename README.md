# StackBlitz ready starter solution with Rust + WASM + Plotly

This project is a small Vite app that renders a Plotly surface chart and calls a Rust-exported `greet()` function from WebAssembly. Vite handles the local dev server and module loading, while the Rust build tooling compiles the wasm package into the checked-in `pkg/` folder so the app can start without a separate rebuild step.

You can run the app directly in StackBlitz from [this GitHub import link](https://stackblitz.com/~/github.com/toreaurstadboss/rust-wasm-plotly). StackBlitz uses the same Vite entry point, so the app behavior matches the local setup as long as the generated wasm files in `pkg/` are present.

The repo includes the following pieces for the wasm workflow:

- `npm run build:wasm` to regenerate the WebAssembly output from the Rust crate.
- Prebuilt wasm artifacts in the `pkg/` folder for quick startup in StackBlitz and local development.
- A Vite-based frontend that imports `pkg/rust_wasm.js` and loads the wasm module at runtime.

## Run locally

1. Install dependencies.

```bash
npm install
```

2. Precompile the Rust/WASM package into `pkg/`.

```bash
npm run build:wasm
```

3. Start the development server.

```bash
npm run dev
```

4. Open the URL shown by Vite, usually `http://localhost:5173`.

## Optional wasm rebuild

If you want to regenerate the Rust/WASM output instead of using the checked-in StackBlitz shim, run:

```bash
npm run build:wasm
```

## StackBlitz

The app can run in StackBlitz with `npm run dev` because the generated wasm module is shimmed in `pkg/rust_wasm.js`. For a local setup, run `npm run build:wasm` first so the `pkg/` output is regenerated before Vite starts.
