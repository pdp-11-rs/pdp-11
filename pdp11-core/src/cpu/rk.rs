use super::*;

/// RK11 Disk Controller for RK05 disk drives
///
/// RK05 disk format:
/// - 203 cylinders
/// - 2 surfaces (heads)
/// - 12 sectors per track
/// - 256 words (512 bytes) per sector
pub struct Rk {
    image_file: PathBuf,
    image: Vec<u8>,
}

// RK11 Register addresses
pub const RKDS: Address<Word> = Address::from_u16(0o177400);
pub const RKER: Address<Word> = Address::from_u16(0o177402);
pub const RKCS: Address<Word> = Address::from_u16(0o177404);
pub const RKWC: Address<Word> = Address::from_u16(0o177406);
pub const RKBA: Address<Word> = Address::from_u16(0o177410);
pub const RKDA: Address<Word> = Address::from_u16(0o177412);

// RKCS bits
const GO: u16 = 0o000001; // Start operation
const FUNC_READ: u16 = 0o000004; // Read function
const READY: u16 = 0o000200; // Controller ready

impl Rk {
    pub fn with_image(image: impl AsRef<Path>) -> io::Result<Self> {
        let image_file = image.as_ref().to_path_buf();
        let image = fs::read(&image_file)?;

        Ok(Self { image_file, image })
    }

    /// Initialize RK11 registers in RAM
    pub fn init_registers(&self, ram: &mut Ram) {
        ram.write_direct(RKDS, Word::from(READY)); // Drive ready
        ram.write_direct(RKER, Word::zero());
        ram.write_direct(RKCS, Word::from(READY)); // Controller ready
        ram.write_direct(RKWC, Word::zero());
        ram.write_direct(RKBA, Word::zero());
        ram.write_direct(RKDA, Word::zero());
    }

    /// Check if a write to RKCS triggered a command, and execute it
    pub fn check_command(&mut self, ram: &mut Ram) {
        let rkcs = ram[RKCS];

        // Check if GO bit is set
        if rkcs.as_u16() & GO != 0 {
            self.execute_command(ram);
        }
    }

    /// Execute RK11 command when GO bit is set
    fn execute_command(&mut self, ram: &mut Ram) {
        let cmd = ram[RKCS].as_u16() & 0o000016; // Function code bits 1-3

        match cmd {
            FUNC_READ => self.read_sector(ram),
            _ => {
                eprintln!("Unsupported RK11 command: {cmd:#08o}");
                ram.write_direct(RKCS, Word::from(READY)); // Set ready, clear GO
            }
        }
    }

    /// Read sector from disk image to memory via DMA
    fn read_sector(&mut self, ram: &mut Ram) {
        // Decode disk address (cylinder/surface/sector)
        let da = ram[RKDA].as_u16();
        let cylinder = (da >> 5) & 0o377; // Bits 5-12: cylinder
        let surface = (da >> 4) & 0o1; // Bit 4: surface (head)
        let sector = da & 0o17; // Bits 0-3: sector

        // Calculate disk offset (256 words = 512 bytes per sector)
        let sector_offset =
            ((cylinder as usize * 2 + surface as usize) * 12 + sector as usize) * 512;

        println!(
            "RK READ: cyl={cylinder:#o} surf={surface} sec={sector:#o} -> offset={sector_offset:#o}"
        );

        // Get word count (negative, counts up to zero)
        let mut wc = ram[RKWC].as_u16();
        let mut ba = ram[RKBA].as_u16();

        // Transfer words from disk to memory
        let mut disk_offset = sector_offset;
        while wc != 0 {
            // Read word from disk image (little-endian)
            if disk_offset + 1 < self.image.len() {
                let low = self.image[disk_offset];
                let high = self.image[disk_offset + 1];
                let word = Word::from(u16::from_le_bytes([low, high]));

                // Write to RAM at bus address
                let addr = Address::<Word>::from_u16(ba);
                ram.write_direct(addr, word);

                disk_offset += 2;
                ba = ba.wrapping_add(2);
                wc = wc.wrapping_add(1); // Word count is negative, counts up
            } else {
                eprintln!("RK READ: disk offset {disk_offset:#o} out of bounds");
                break;
            }
        }

        // Update registers after transfer
        ram.write_direct(RKWC, Word::from(wc));
        ram.write_direct(RKBA, Word::from(ba));
        ram.write_direct(RKCS, Word::from(READY)); // Set ready, clear GO
    }
}

impl fmt::Debug for Rk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rk")
            .field("image", &self.image_file)
            .finish()
    }
}
