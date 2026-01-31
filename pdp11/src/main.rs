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
