use std::io;

use pdp11_core::cpu;

fn main() -> io::Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let core = cpu::Cpu::new("rk0.img")?;
    core.poweron();
    Ok(())
}
