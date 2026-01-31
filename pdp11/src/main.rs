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

    // Check if we should exit on prompt detection (for testing/CI)
    let exit_on_prompt = std::env::var("PDP11_EXIT_ON_PROMPT")
        .is_ok_and(|v| v == "1" || v.eq_ignore_ascii_case("true"));

    if exit_on_prompt {
        tracing::info!("PDP11_EXIT_ON_PROMPT is set - will exit when boot completes");
    } else {
        tracing::info!("Running normally - use Ctrl+C to exit");
    }

    // Run until halted, with progress reporting
    let mut instruction_count = 0u64;
    let report_interval = 100_000;
    let check_input_interval = 1000; // Check for input every 1000 instructions
    let mut last_pcs: Vec<u16> = Vec::with_capacity(1000);
    let mut prompt_detected = false;

    loop {
        if core.is_halted() {
            tracing::info!("CPU halted after {} instructions", instruction_count);
            break;
        }
        core.step();
        instruction_count += 1;

        // Check for console input periodically
        if instruction_count.is_multiple_of(check_input_interval) {
            core.check_console_input();
        }

        // Track recent PCs to detect tight loops (console input wait)
        let pc = core.pc_value().as_u16();
        last_pcs.push(pc);

        // Check every 1000 instructions if we're in a tight loop
        if !prompt_detected && last_pcs.len() >= 1000 {
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

                prompt_detected = true;

                // Only exit if requested (for testing), otherwise continue running
                if exit_on_prompt {
                    break;
                }
            }

            // Reset tracking window
            last_pcs.clear();
        }

        if instruction_count.is_multiple_of(report_interval) && !prompt_detected {
            tracing::info!("Executed {} instructions...", instruction_count);
        }
    }

    tracing::info!("Emulator stopped");

    Ok(())
}
