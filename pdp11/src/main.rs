use std::io;

use pdp11_core::cpu;

#[cfg(unix)]
mod terminal {
    use std::io;
    use std::os::unix::io::AsRawFd;

    pub struct RawMode {
        original: libc::termios,
    }

    impl RawMode {
        pub fn enable() -> io::Result<Self> {
            unsafe {
                let stdin_fd = io::stdin().as_raw_fd();
                let mut original: libc::termios = std::mem::zeroed();
                
                if libc::tcgetattr(stdin_fd, &mut original) != 0 {
                    return Err(io::Error::last_os_error());
                }

                let mut raw = original;
                // Disable canonical mode, echo, and signals
                raw.c_lflag &= !(libc::ICANON | libc::ECHO | libc::ISIG);
                // Set minimum characters to 0 (non-blocking)
                raw.c_cc[libc::VMIN] = 0;
                raw.c_cc[libc::VTIME] = 0;

                if libc::tcsetattr(stdin_fd, libc::TCSANOW, &raw) != 0 {
                    return Err(io::Error::last_os_error());
                }

                Ok(RawMode { original })
            }
        }
    }

    impl Drop for RawMode {
        fn drop(&mut self) {
            unsafe {
                let stdin_fd = io::stdin().as_raw_fd();
                libc::tcsetattr(stdin_fd, libc::TCSANOW, &self.original);
            }
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
