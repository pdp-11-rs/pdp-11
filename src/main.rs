use std::fmt;
use std::fs;
use std::io;
use std::ops;
use std::path::{Path, PathBuf};

mod cpu;

fn main() -> io::Result<()> {
    let core = cpu::Cpu::new("rk0.img")?;
    core.poweron();
    Ok(())
}
