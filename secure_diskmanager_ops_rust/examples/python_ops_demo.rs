use secure_diskmanager_ops::python_ops;

fn main() -> secure_diskmanager_ops::Result<()> {
    println!("represented demos: {}", python_ops::represented_python_demos().len());
    print!("{}", python_ops::demo_report()?);

    let svg = python_ops::line_chart_svg(
        "Python demo chart equivalent",
        &["A", "B", "C"],
        &[1.0, 4.0, 9.0],
        320,
        200,
    )?;
    println!("chart svg bytes: {}", svg.len());

    let table = python_ops::dataframe_to_table(&[
        python_ops::DataRow { x: 1, y: 10 },
        python_ops::DataRow { x: 2, y: 20 },
        python_ops::DataRow { x: 3, y: 30 },
    ]);
    println!("{table}");

    Ok(())
}
