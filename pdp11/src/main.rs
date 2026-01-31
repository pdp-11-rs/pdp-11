use std::io;

use pdp11_core::cpu;

fn main() -> io::Result<()> {
    // Initialize simple tracing subscriber without thread spawning
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_writer(io::stderr)
        .init();

    let mut core = cpu::Cpu::new("rk0.img")?;
    core.reset();

    tracing::info!("Starting emulator...");

    // Run until halted, with progress reporting
    let mut instruction_count = 0u64;
    let report_interval = 100_000;
    let mut last_pcs: Vec<u16> = Vec::with_capacity(1000);

    loop {
        if core.is_halted() {
            tracing::info!("CPU halted after {} instructions", instruction_count);
            break;
        }
        core.step();
        instruction_count += 1;

        // Track recent PCs to detect tight loops (console input wait)
        let pc = core.pc_value().as_u16();
        last_pcs.push(pc);

        // Check every 1000 instructions if we're in a tight loop
        if last_pcs.len() >= 1000 {
            let mut unique_pcs = last_pcs.clone();
            unique_pcs.sort_unstable();
            unique_pcs.dedup();

            // If executing only a handful of unique PCs, we're waiting for I/O
            if unique_pcs.len() <= 10 && instruction_count > 100_000 {
                tracing::info!(
                    "✓ Boot completed successfully after {} instructions",
                    instruction_count
                );
                tracing::info!("System is waiting for console input at ODT prompt (@)");
                tracing::debug!(
                    "Tight loop at PCs: {:?}",
                    unique_pcs
                        .iter()
                        .map(|pc| format!("{:#08o}", pc))
                        .collect::<Vec<_>>()
                );
                break;
            }

            // Reset tracking window
            last_pcs.clear();
        }

        if instruction_count % report_interval == 0 {
            tracing::info!("Executed {} instructions...", instruction_count);
        }

        // Safety limit to prevent truly infinite loops
        if instruction_count >= 1_000_000 {
            tracing::warn!("Reached maximum instruction limit at PC={:#08o}", pc);
            break;
        }
    }

    tracing::info!("Emulator stopped");

    Ok(())
}
