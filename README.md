# Rust + WASM + Plotly

This is a small Vite app that renders a Plotly surface chart and calls a Rust-exported `greet()` function.

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

If you want to regenerate the Rust/WASM output instead of using the checked-in StackBlitz shim, run:

```bash
npm run build:wasm
```

## StackBlitz

The app can run in StackBlitz with `npm run dev` because the generated wasm module is shimmed in `pkg/rust_wasm.js` until the Rust build is regenerated locally.