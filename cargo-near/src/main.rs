use cargo_near_lib::build;
use color_eyre::eyre::Context;

fn main() -> color_eyre::Result<()> {

    #[cfg(not(debug_assertions))]
    let display_env_section = false;
    #[cfg(debug_assertions)]
    let display_env_section = true;
    color_eyre::config::HookBuilder::default()
        .display_env_section(display_env_section)
        .install()?;

    build(123, 123123).wrap_err("new context in binary")?;
    println!("hello");
    Ok(())

}
