use crate::error::Result;

pub trait Cipher {
    fn encrypt(&self, text: &str) -> Result<String>;
    /// # Notes
    /// When the function is not implemented,
    /// it is implied, that decoding works the same way as encoding.
    fn decrypt(&self, text: &str) -> Result<String> {
        self.encrypt(text)
    }
}
