use std::io;

use pdp11_core::cpu;

fn main() -> io::Result<()> {
    let core = cpu::Cpu::new("rk0.img")?;
    core.poweron();
    Ok(())
}
