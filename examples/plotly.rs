use plotly::{
    Layout, Plot, Scatter,
    common::{Mode, Title},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("target/plots")?;

    let x_values = (0..1000)
        .map(|i| i as f64 * 10.0 / 999.0)
        .collect::<Vec<_>>();
    let sin_values = x_values.iter().map(|x| x.sin()).collect::<Vec<_>>();
    let cos_values = x_values.iter().map(|x| x.cos()).collect::<Vec<_>>();

    let sin_trace = Scatter::new(x_values.clone(), sin_values)
        .mode(Mode::Lines)
        .name("sin(x)");
    let cos_trace = Scatter::new(x_values, cos_values)
        .mode(Mode::Lines)
        .name("cos(x)");

    let mut plot = Plot::new();
    plot.add_trace(sin_trace);
    plot.add_trace(cos_trace);
    plot.set_layout(Layout::new().title(Title::with_text("Sine and cosine")));

    let output_path = "target/plots/sine_cosine_plotly.html";
    plot.write_html(output_path);
    println!("Wrote {output_path}");

    Ok(())
}
