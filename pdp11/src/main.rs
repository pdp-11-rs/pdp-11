use std::io;

use pdp11_core::cpu;

fn main() -> io::Result<()> {
    // Initialize tracing subscriber with environment-based filtering
    // Defaults to INFO level if RUST_LOG is not set
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let mut core = cpu::Cpu::new("rk0.img")?;
    core.reset();

    tracing::info!("Starting emulator...");

    // Run for a limited number of instructions for testing
    let max_instructions = 1000;
    for i in 0..max_instructions {
        if core.is_halted() {
            tracing::info!("CPU halted after {} instructions", i);
            break;
        }
        core.step();
    }

    tracing::info!("Emulator stopped");

    Ok(())
}
