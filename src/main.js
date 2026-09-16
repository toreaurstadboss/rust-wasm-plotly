import Plotly from "plotly.js-dist-min";
import init, { greet } from "../pkg/rust_wasm.js";

init().then(() => {
  greet();

  Plotly.newPlot("plot", [{
    z: [[1, 2], [3, 4]],
    type: "surface"
  }]);
});
