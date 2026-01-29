use super::*;

pub struct Rk {
    image_file: PathBuf,
    image: Vec<u8>,
}

impl Rk {
    pub fn with_image(image: impl AsRef<Path>) -> io::Result<Self> {
        let image_file = image.as_ref().to_path_buf();
        fs::read(image).map(|image| Self { image_file, image })
    }

    fn read_word(&self, addr: Address<Word>) -> Word {
        todo!()
    }
}

impl fmt::Debug for Rk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rk")
            .field("image", &self.image_file)
            .finish()
    }
}
