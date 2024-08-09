use cargo_near_lib::build_extended;

fn main() -> color_eyre::Result<()> {

    color_eyre::install()?;

    build_extended(123, 123123, "middle of nowhre".into())?;
    println!("hello");
    Ok(())

}
