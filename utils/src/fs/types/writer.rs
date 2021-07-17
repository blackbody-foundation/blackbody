use super::*;

pub struct Writer<'a, T> {
    fm: &'a FM<T>,
    ptr: Box<BufWriter<File>>,
}
impl<'a, T> Writer<'a, T>
where
    T: HeaderTrait,
{
    pub fn new(fm: &'a FM<T>, ptr: &Ptr) -> Result<Self> {
        let ptr = ptr.to_writer()?;
        Ok(Self { fm, ptr })
    }
    pub fn set_cursor(&mut self, pos: uPS) -> Result<uPS> {
        Ok(
            errors::handle_io_error(self.ptr.seek(SeekFrom::Start(pos + self.fm.header_size)))?
                - self.fm.header_size as uPS,
        )
    }
    pub fn set_cursor_relative(&mut self, pos: iPS) -> Result<uPS> {
        Ok(Self::err_tunnel(
            self.ptr
                .seek(SeekFrom::Current(pos + self.fm.header_size as iPS)),
        )? - self.fm.header_size as uPS)
    }
    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        Self::err_tunnel(self.ptr.write_all(buf))
    }
    pub fn write_cursoring(&mut self, buf: &[u8], pos: uPS) -> Result<()> {
        self.set_cursor(pos)?;
        Self::err_tunnel(self.ptr.write_all(buf))
    }
    fn err_tunnel<E>(io_e: std::io::Result<E>) -> Result<E> {
        errors::handle_io_error(io_e)
    }
}
