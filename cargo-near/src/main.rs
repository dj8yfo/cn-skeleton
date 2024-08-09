use cargo_near_lib::build;

fn main() -> color_eyre::Result<()> {

    color_eyre::install()?;

    build(123, 123123)?;
    println!("hello");
    Ok(())

}
