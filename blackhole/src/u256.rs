pub struct U256 {
    buffer: [u8; 32],
}
impl U256 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn as_mut_u8(&mut self) -> &mut [u8; 32] {
        &mut self.buffer
    }
    pub fn as_u8(&self) -> &[u8; 32] {
        &self.buffer
    }
    pub fn as_u8_slice(&self, len: usize) -> &[u8] {
        &self.buffer[..len]
    }
}
impl Default for U256 {
    fn default() -> Self {
        Self { buffer: [0; 32] }
    }
}
