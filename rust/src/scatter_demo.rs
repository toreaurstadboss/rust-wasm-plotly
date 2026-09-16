use rand::Rng;
use serde_json::json;

const POINT_COUNT: usize = 10;
const SERIES_COUNT: usize = 3;
const Y_MIN: i32 = -80;
const Y_MAX: i32 = 80;

pub fn build_scatter_demo_json() -> String {
    json!({
        "data": build_traces(),
        "layout": build_layout(),
    })
    .to_string()
}

fn build_traces() -> Vec<serde_json::Value> {
    (1..=SERIES_COUNT)
        .map(|series_index| build_trace(&format!("Series {series_index}"), series_index == 1))
        .collect()
}

fn build_trace(series_name: &str, show_scale: bool) -> serde_json::Value {
    let x_values = build_x_values();
    let y_values = build_y_values();
    let labels = build_labels(&x_values, &y_values);

    json!({
        "x": x_values,
        "y": y_values,
        "mode": "lines+markers+text",
        "type": "scatter",
        "name": series_name,
        "text": labels,
        "textposition": "top center",
        "hovertemplate": "(%{x}, %{y})<extra></extra>",
        "textfont": {
            "family": "Aptos, Segoe UI, Arial, sans-serif",
            "size": 11,
            "color": "#E2E8F0"
        },
        "marker": {
            "size": 10,
            "color": y_values,
            "colorscale": "Viridis",
            "showscale": show_scale,
            "colorbar": {
                "title": {
                    "text": "Intensity (Y-value)"
                }
            }
        }
    })
}

fn build_layout() -> serde_json::Value {
    json!({
        "title": {
            "text": "2D scatter plot with Plotly",
            "font": {
                "family": "Aptos, Segoe UI, Arial, sans-serif",
                "size": 24,
                "color": "#F8FAFC"
            }
        },
        "font": {
            "family": "Aptos, Segoe UI, Arial, sans-serif",
            "size": 14,
            "color": "#E2E8F0"
        },
        "legend": {
            "orientation": "h",
            "x": 0,
            "xanchor": "left",
            "y": -0.22
        },
        "margin": {
            "t": 80,
            "r": 30,
            "b": 80,
            "l": 70
        },
        "paper_bgcolor": "rgba(0, 0, 0, 0)",
        "plot_bgcolor": "rgba(15, 23, 42, 0.82)",
        "xaxis": {
            "title": {
                "text": "X Axis",
                "font": {
                    "family": "Aptos, Segoe UI, Arial, sans-serif",
                    "size": 16,
                    "color": "#F8FAFC"
                }
            },
            "dtick": 1,
            "showgrid": true,
            "gridcolor": "rgba(148, 163, 184, 0.35)"
        },
        "yaxis": {
            "title": {
                "text": "Y Axis",
                "font": {
                    "family": "Aptos, Segoe UI, Arial, sans-serif",
                    "size": 16,
                    "color": "#F8FAFC"
                }
            },
            "dtick": 10,
            "showgrid": true,
            "gridcolor": "rgba(148, 163, 184, 0.35)"
        }
    })
}

fn build_x_values() -> Vec<i32> {
    (1..=POINT_COUNT as i32).collect()
}

fn build_y_values() -> Vec<i32> {
    let mut rng = rand::thread_rng();

    (0..POINT_COUNT)
        .map(|_| rng.gen_range(Y_MIN..=Y_MAX))
        .collect()
}

fn build_labels(x_values: &[i32], y_values: &[i32]) -> Vec<String> {
    x_values
        .iter()
        .zip(y_values.iter())
        .map(|(x_value, y_value)| format!("({x_value}, {y_value})"))
        .collect()
}