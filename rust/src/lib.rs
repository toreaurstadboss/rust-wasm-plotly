mod scatter_demo;

use wasm_bindgen::prelude::*;

/// Returns the Plotly scatter demo as JSON for the browser loader.
#[wasm_bindgen]
pub fn scatter_demo_json() -> String {
    scatter_demo::build_scatter_demo_json()
}
