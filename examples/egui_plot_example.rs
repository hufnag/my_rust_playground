use eframe::egui;
use egui_plot::{Legend, Line, Plot};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "egui plot example",
        options,
        Box::new(|_creation_context| Ok(Box::new(PlotApp::new()))),
    )
}

struct PlotApp {
    sin_points: Vec<[f64; 2]>,
    cos_points: Vec<[f64; 2]>,
}

impl PlotApp {
    fn new() -> Self {
        let x_values = (0..1000).map(|i| i as f64 * 10.0 / 999.0);
        let sin_points = x_values.clone().map(|x| [x, x.sin()]).collect();
        let cos_points = x_values.map(|x| [x, x.cos()]).collect();

        Self {
            sin_points,
            cos_points,
        }
    }
}

impl eframe::App for PlotApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        Plot::new("sine_cosine")
            .legend(Legend::default())
            .x_axis_label("x")
            .y_axis_label("y")
            .view_aspect(1.7)
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new("sin(x)", self.sin_points.clone()));
                plot_ui.line(Line::new("cos(x)", self.cos_points.clone()));
            });
    }
}
