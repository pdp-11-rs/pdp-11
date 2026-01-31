use crate::devices::*;

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
    // RK11 Registers
    rkds: Word, // Drive Status
    rker: Word, // Error Register
    rkcs: Word, // Control Status
    rkwc: Word, // Word Count
    rkba: Word, // Bus Address
    rkda: Word, // Disk Address
}

// RK11 Register addresses
pub const RKDS: Address<Word> = Address::from_u16(0o177400);
pub const RKER: Address<Word> = Address::from_u16(0o177402);
pub const RKCS: Address<Word> = Address::from_u16(0o177404);
pub const RKWC: Address<Word> = Address::from_u16(0o177406);
pub const RKBA: Address<Word> = Address::from_u16(0o177410);
pub const RKDA: Address<Word> = Address::from_u16(0o177412);

// RKCS bits
const GO: Word = Word::from_u16(0o000001); // Start operation
const FUNC_READ: Word = Word::from_u16(0o000004); // Read function
const READY: Word = Word::from_u16(0o000200); // Controller ready

// RK05 disk geometry
const RK05_CYLINDERS: usize = 203;
const RK05_SURFACES: usize = 2;
const RK05_SECTORS_PER_TRACK: usize = 12;
const RK05_BYTES_PER_SECTOR: usize = 512;
const RK05_TOTAL_SIZE: usize =
    RK05_CYLINDERS * RK05_SURFACES * RK05_SECTORS_PER_TRACK * RK05_BYTES_PER_SECTOR;

impl Rk {
    pub fn with_image(image: impl AsRef<Path>) -> io::Result<Self> {
        let image_file = image.as_ref().to_path_buf();
        let image = fs::read(&image_file)?;

        // Validate disk image size
        if image.len() != RK05_TOTAL_SIZE {
            tracing::warn!(
                "RK05 disk image size mismatch: expected {} bytes ({} MB), got {} bytes ({:.2} MB)",
                RK05_TOTAL_SIZE,
                RK05_TOTAL_SIZE / (1024 * 1024),
                image.len(),
                image.len() as f64 / (1024.0 * 1024.0)
            );
            tracing::warn!(
                "RK05 geometry: {} cylinders × {} surfaces × {} sectors × {} bytes",
                RK05_CYLINDERS,
                RK05_SURFACES,
                RK05_SECTORS_PER_TRACK,
                RK05_BYTES_PER_SECTOR
            );
        }

        Ok(Self {
            image_file,
            image,
            rkds: READY,
            rker: Word::zero(),
            rkcs: READY,
            rkwc: Word::zero(),
            rkba: Word::zero(),
            rkda: Word::zero(),
        })
    }

    /// Create an empty RK disk for testing purposes
    /// Creates a minimal 512-byte disk image with no file backing
    #[cfg(test)]
    pub fn empty() -> Self {
        Self {
            image_file: PathBuf::from("<test>"),
            image: vec![0u8; 512],
            rkds: READY,
            rker: Word::zero(),
            rkcs: READY,
            rkwc: Word::zero(),
            rkba: Word::zero(),
            rkda: Word::zero(),
        }
    }

    /// Initialize RK11 registers in RAM
    pub fn init_registers(&self, ram: &mut Ram) {
        ram.write_direct(RKDS, self.rkds);
        ram.write_direct(RKER, self.rker);
        ram.write_direct(RKCS, self.rkcs);
        ram.write_direct(RKWC, self.rkwc);
        ram.write_direct(RKBA, self.rkba);
        ram.write_direct(RKDA, self.rkda);
    }

    /// Execute any pending RK11 command (called after RKCS write)
    pub fn execute_pending_command(&mut self, ram: &mut Ram) {
        self.check_command(ram);
    }

    /// Check if a write to RKCS triggered a command, and execute it
    fn check_command(&mut self, ram: &mut Ram) {
        // Check if GO bit is set
        if (self.rkcs & GO) != Word::zero() {
            self.execute_command(ram);
        }
    }

    /// Execute RK11 command when GO bit is set
    fn execute_command(&mut self, ram: &mut Ram) {
        let cmd = self.rkcs & Word::from_u16(0o000016); // Function code bits 1-3

        match cmd {
            FUNC_READ => self.read_sector(ram),
            _ => {
                tracing::warn!("Unsupported RK11 command: {:#08o}", cmd.as_u16());
                self.rkcs = READY; // Set ready, clear GO
            }
        }
    }

    /// Read sector from disk image to memory via DMA
    fn read_sector(&mut self, ram: &mut Ram) {
        // Decode disk address (cylinder/surface/sector)
        let da = self.rkda.as_u16();
        let cylinder = (da >> 5) & 0o377; // Bits 5-12: cylinder
        let surface = (da >> 4) & 0o1; // Bit 4: surface (head)
        let sector = da & 0o17; // Bits 0-3: sector

        // Calculate disk offset (256 words = 512 bytes per sector)
        let sector_offset =
            ((cylinder as usize * 2 + surface as usize) * 12 + sector as usize) * 512;

        tracing::debug!(
            "RK READ: cyl={cylinder:#o} surf={surface} sec={sector:#o} -> offset={sector_offset:#o}"
        );

        // Get word count (negative, counts up to zero)
        let mut wc = self.rkwc.as_u16();
        let mut ba = self.rkba.as_u16();

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
                tracing::error!("RK READ: disk offset {disk_offset:#o} out of bounds");
                break;
            }
        }

        // Update registers after transfer
        self.rkwc = Word::from(wc);
        self.rkba = Word::from(ba);
        self.rkcs = READY; // Set ready, clear GO
    }
}

impl MmioDevice for Rk {
    fn read_word(&mut self, address: Address<Word>) -> Word {
        match address {
            RKDS => self.rkds,
            RKER => self.rker,
            RKCS => self.rkcs,
            RKWC => self.rkwc,
            RKBA => self.rkba,
            RKDA => self.rkda,
            _ => Word::zero(),
        }
    }

    fn write_word(&mut self, address: Address<Word>, value: Word) {
        match address {
            RKDS => self.rkds = value,
            RKER => self.rker = value,
            RKCS => {
                self.rkcs = value;
                // Note: check_command will be called separately by CPU
            }
            RKWC => self.rkwc = value,
            RKBA => self.rkba = value,
            RKDA => self.rkda = value,
            _ => {}
        }
    }

    fn address_range(&self) -> (u16, u16) {
        (0o177400, 0o177412)
    }

    fn handles_word_address(&self, address: Address<Word>) -> bool {
        matches!(address, RKDS | RKER | RKCS | RKWC | RKBA | RKDA)
    }

    fn handles_byte_address(&self, address: Address<Byte>) -> bool {
        let start = Address::<Byte>::from_u16(0o177400);
        let end = Address::<Byte>::from_u16(0o177413); // RKDA + 1
        address >= start && address <= end
    }
}

impl fmt::Debug for Rk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rk")
            .field("image", &self.image_file)
            .finish()
    }
}
