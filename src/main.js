import Plotly from "plotly.js-dist-min";
import init, { scatter_demo_json } from "../pkg/rust_wasm.js";

const plotContainer = document.getElementById("plot");

document.body.style.margin = "0";
document.body.style.minHeight = "100vh";
document.body.style.background = "linear-gradient(135deg, #081120 0%, #132238 45%, #050816 100%)";
document.body.style.fontFamily = "Aptos, Segoe UI, Arial, sans-serif";

plotContainer.style.width = "min(1100px, calc(100vw - 2rem))";
plotContainer.style.height = "min(720px, calc(100vh - 3rem))";
plotContainer.style.margin = "1rem auto";
plotContainer.style.borderRadius = "18px";
plotContainer.style.boxShadow = "0 20px 48px rgba(15, 23, 42, 0.45)";

const shuffleButton = document.createElement("button");
shuffleButton.type = "button";
shuffleButton.textContent = "Shuffle series";
shuffleButton.style.position = "fixed";
shuffleButton.style.top = "16px";
shuffleButton.style.left = "16px";
shuffleButton.style.zIndex = "1000";
shuffleButton.style.padding = "10px 14px";
shuffleButton.style.border = "0";
shuffleButton.style.borderRadius = "10px";
shuffleButton.style.background = "#38bdf8";
shuffleButton.style.color = "#081120";
shuffleButton.style.font = '600 14px "Aptos", "Segoe UI", Arial, sans-serif';
shuffleButton.style.cursor = "pointer";
shuffleButton.style.boxShadow = "0 10px 24px rgba(15, 23, 42, 0.35)";
document.body.appendChild(shuffleButton);

function renderPlot() {
  const demoSpec = JSON.parse(scatter_demo_json());

  return Plotly.newPlot(plotContainer, demoSpec.data, demoSpec.layout, {
    responsive: true
  });
}

function shuffleSeries() {
  const demoSpec = JSON.parse(scatter_demo_json());

  Plotly.react(plotContainer, demoSpec.data, demoSpec.layout, {
    responsive: true
  });
}

shuffleButton.addEventListener("click", shuffleSeries);
window.addEventListener("resize", () => Plotly.Plots.resize(plotContainer));

init().then(() => {
  renderPlot();
});
