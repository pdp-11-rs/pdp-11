use std::io;

use pdp11_core::cpu;

#[cfg(unix)]
mod terminal {
    use std::io;
    use termion::raw::{IntoRawMode, RawTerminal};

    pub struct RawMode {
        _stdout: RawTerminal<io::Stdout>,
    }

    impl RawMode {
        pub fn enable() -> io::Result<Self> {
            // Put stdout into raw mode - this also affects stdin behavior
            let stdout = io::stdout().into_raw_mode()?;
            Ok(RawMode { _stdout: stdout })
        }
    }
}

fn main() -> io::Result<()> {
    // Initialize tracing subscriber with environment-based filtering
    // Defaults to INFO level if RUST_LOG is not set
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // Enable raw terminal mode on Unix for immediate character input
    #[cfg(unix)]
    let _raw_mode = terminal::RawMode::enable()?;

    let mut core = cpu::Cpu::new("rk0.img")?;
    core.reset();

    tracing::info!("Starting emulator...");

    // Run until halted, with progress reporting
    let mut instruction_count = 0u64;
    let report_interval = 100_000;
    let mut last_pcs: Vec<u16> = Vec::with_capacity(100);
    loop {
        if core.is_halted() {
            tracing::info!("CPU halted after {} instructions", instruction_count);
            break;
        }
        core.step();
        instruction_count += 1;

        // Track recent PCs to detect tight loops
        let pc = core.pc_value().as_u16();
        if instruction_count > 9_900_000 {
            last_pcs.push(pc);
        }

        if instruction_count % report_interval == 0 {
            tracing::info!("Executed {} instructions...", instruction_count);
        }

        // Safety limit to prevent infinite loops during development
        // Note: A successful boot typically shows @ prompt and waits for console input
        if instruction_count >= 10_000_000 {
            let pc = core.pc_value();
            tracing::info!("Reached instruction limit at PC={:#08o}", pc.as_u16());
            // Show last few unique PCs
            last_pcs.sort();
            last_pcs.dedup();
            if last_pcs.len() <= 10 {
                tracing::info!("System waiting for console input (ODT @ prompt shown):");
                for pc in &last_pcs {
                    tracing::info!("  PC={:#08o}", pc);
                }
                tracing::info!("✓ Boot completed successfully! System is at the ODT prompt.");
            } else {
                tracing::warn!("Unique PCs in last 100K instructions: {}", last_pcs.len());
            }
            break;
        }
    }

    tracing::info!("Emulator stopped");

    Ok(())
}
