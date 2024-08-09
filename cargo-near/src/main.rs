use cargo_near_lib::build_extended;

fn main() -> color_eyre::Result<()> {

    color_eyre::install()?;

    build_extended(123, 123123, "./fakey_fakey_path".into())?;
    println!("hello");
    Ok(())

}
