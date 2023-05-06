mod cpu;

fn main() {
    let core = cpu::Cpu::new();
    core.run();
}
