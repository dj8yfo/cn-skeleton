use cargo_near_lib::build_extended;
use color_eyre::Section;
use tracing::instrument;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use colored::Colorize;

fn install_tracing() {
    use tracing_error::ErrorLayer;

    tracing_subscriber::registry()
        .with(ErrorLayer::default())
        .init();
}

fn main() -> color_eyre::Result<()> {
    if std::env::var("RUST_SPANTRACE").is_err() {
        std::env::set_var("RUST_SPANTRACE", "0");
    }
    install_tracing();
    #[cfg(not(debug_assertions))]
    color_eyre::config::HookBuilder::default()
        .add_frame_filter(Box::new(|frames| frames.retain(|_frame| false)))
        .display_env_section(false)
        .install()?;
    #[cfg(debug_assertions)]
    color_eyre::config::HookBuilder::default()
        .display_env_section(true)
        .install()?;
    some_higher_level_build(123, 123123, "./fakey_fakey_path".into())
}

#[instrument]
fn some_higher_level_build(left: u64, right: u64, middle: String) -> color_eyre::Result<()> {
    match build_extended(left, right, middle) {
        Ok(..) => {

        
        },
        Err(err) => {
            let err = color_eyre::eyre::Report::from(err)
                .suggestion("try to use good good file".cyan())
                .warning("what ever happens next is good".yellow())
                .suggestion("better".cyan())
                .note("Cargo.lock check was performed against git version of code.".cyan())
                .note("Don't forget to check in Cargo.lock into source code for deploy if it's git-ignored...    ".cyan());
            return Err(err);
        },
        
    };
    Ok(())
}
