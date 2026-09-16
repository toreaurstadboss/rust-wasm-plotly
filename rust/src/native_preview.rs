const PAGE_BACKGROUND_STYLE: &str =
    "margin:0; min-height:100vh; background: linear-gradient(135deg, #081120 0%, #132238 45%, #050816 100%); font-family: 'Aptos', 'Segoe UI', Arial, sans-serif;";
const HTML_TEMPLATE: &str = r##"<!doctype html>
<html lang="en">

<head>
    <meta charset="utf-8" />
    <script src="https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg.js"></script>
    <script src="https://cdn.plot.ly/plotly-3.0.1.min.js"></script>
</head>

<body style="{PAGE_BACKGROUND_STYLE}">
    <div>
        <div id="plotly-html-element" class="plotly-graph-div" style="height:100%; width:50%; margin:0 auto;"></div>
    </div>
    <button
      id="shuffle-button"
      style="
        position: fixed;
        top: 16px;
        left: 16px;
        z-index: 1000;
        padding: 10px 14px;
        border: 0;
        border-radius: 10px;
        background: #38bdf8;
        color: #081120;
        font:
          600 14px &quot;Aptos&quot;,
          &quot;Segoe UI&quot;,
          Arial,
          sans-serif;
        cursor: pointer;
        box-shadow: 0 10px 24px rgba(15, 23, 42, 0.35);
      "
    >
      Shuffle series
    </button>
    <script>
      const graphDiv = document.getElementById("plotly-html-element");
      const pointCount = 10;
      const yMin = -80;
      const yMax = 80;

      function buildSeries(seriesName, showScale) {
        const xValues = Array.from({ length: pointCount }, (_, index) => index + 1);
        const yValues = Array.from({ length: pointCount }, () => Math.floor(Math.random() * (yMax - yMin + 1)) + yMin);

        return {
          x: xValues,
          y: yValues,
          mode: "lines+markers+text",
          type: "scatter",
          name: seriesName,
          text: xValues.map((xValue, index) => `(${xValue}, ${yValues[index]})`),
          textposition: "top center",
          hovertemplate: "(%{x}, %{y})<extra></extra>",
          textfont: {
            family: "Aptos, Segoe UI, Arial, sans-serif",
            size: 11,
            color: "#E2E8F0"
          },
          marker: {
            size: 10,
            color: yValues,
            colorscale: "Viridis",
            showscale: showScale,
            colorbar: {
              title: {
                text: "Intensity (Y-value)"
              }
            }
          }
        };
      }

      function buildLayout() {
        return {
          title: {
            text: "2D scatter plot with Plotly",
            font: {
              family: "Aptos, Segoe UI, Arial, sans-serif",
              size: 24,
              color: "#F8FAFC"
            }
          },
          font: {
            family: "Aptos, Segoe UI, Arial, sans-serif",
            size: 14,
            color: "#E2E8F0"
          },
          legend: {
            orientation: "h",
            x: 0,
            xanchor: "left",
            y: -0.22
          },
          margin: {
            t: 80,
            r: 30,
            b: 80,
            l: 70
          },
          paper_bgcolor: "rgba(0, 0, 0, 0)",
          plot_bgcolor: "rgba(15, 23, 42, 0.82)",
          xaxis: {
            title: {
              text: "X Axis",
              font: {
                family: "Aptos, Segoe UI, Arial, sans-serif",
                size: 16,
                color: "#F8FAFC"
              }
            },
            dtick: 1,
            showgrid: true,
            gridcolor: "rgba(148, 163, 184, 0.35)"
          },
          yaxis: {
            title: {
              text: "Y Axis",
              font: {
                family: "Aptos, Segoe UI, Arial, sans-serif",
                size: 16,
                color: "#F8FAFC"
              }
            },
            dtick: 10,
            showgrid: true,
            gridcolor: "rgba(148, 163, 184, 0.35)"
          }
        };
      }

      function renderPlot() {
        const demoSpec = {
          data: [
            buildSeries("Series 1", true),
            buildSeries("Series 2", false),
            buildSeries("Series 3", false)
          ],
          layout: buildLayout()
        };

        Plotly.newPlot(graphDiv, demoSpec.data, demoSpec.layout, { responsive: true });
      }

      function shuffleSeries() {
        const demoSpec = {
          data: [
            buildSeries("Series 1", true),
            buildSeries("Series 2", false),
            buildSeries("Series 3", false)
          ],
          layout: buildLayout()
        };

        Plotly.react(graphDiv, demoSpec.data, demoSpec.layout, { responsive: true });
      }

      renderPlot();

      document.getElementById("shuffle-button").addEventListener("click", () => {
        shuffleSeries();
      });
    </script>
</body>

</html>
"##;

pub fn build_scatter_demo_html() -> String {
    HTML_TEMPLATE
        .replace("{PAGE_BACKGROUND_STYLE}", PAGE_BACKGROUND_STYLE)
}