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

    // Run until halted, with progress reporting
    let mut instruction_count = 0u64;
    let report_interval = 100_000;
    loop {
        if core.is_halted() {
            tracing::info!("CPU halted after {} instructions", instruction_count);
            break;
        }
        core.step();
        instruction_count += 1;

        if instruction_count % report_interval == 0 {
            tracing::info!("Executed {} instructions...", instruction_count);
        }

        // Safety limit to prevent infinite loops during development
        if instruction_count >= 10_000_000 {
            tracing::warn!("Reached safety limit of {} instructions", instruction_count);
            break;
        }
    }

    tracing::info!("Emulator stopped");

    Ok(())
}
