use cargo_near_lib::build;
use color_eyre::eyre::Context;

fn main() -> color_eyre::Result<()> {

    color_eyre::install()?;

    build(123, 123123).wrap_err("new context in binary")?;
    println!("hello");
    Ok(())

}
