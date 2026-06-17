use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("target/plots")?;

    let output_path = "target/plots/sine_cosine.png";
    let root = BitMapBackend::new(output_path, (900, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Sine and cosine", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0.0f64..10.0f64, -1.2f64..1.2f64)?;

    chart.configure_mesh().x_desc("x").y_desc("y").draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..1000).map(|i| {
                let x = i as f64 * 10.0 / 999.0;
                (x, x.sin())
            }),
            &BLUE,
        ))?
        .label("sin(x)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    chart
        .draw_series(LineSeries::new(
            (0..1000).map(|i| {
                let x = i as f64 * 10.0 / 999.0;
                (x, x.cos())
            }),
            &RED,
        ))?
        .label("cos(x)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    root.present()?;
    println!("Wrote {output_path}");

    Ok(())
}
