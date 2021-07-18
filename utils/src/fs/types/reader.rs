// use super::*;

// pub struct Reader<'a, T> {
//     fm: &'a FM<T>,
//     ptr: Box<BufReader<File>>,
// }
// impl<'a, T> Reader<'a, T>
// where
//     T: HeaderTrait,
// {
//     pub fn new(fm: &'a FM<T>, ptr: &Ptr) -> Result<Self> {
//         let ptr = ptr.to_reader()?;
//         Ok(Self { fm, ptr })
//     }
//     pub fn set_cursor(&mut self, pos: uPS) -> Result<uPS> {
//         Ok(
//             Self::err_tunnel(self.ptr.seek(SeekFrom::Start(pos + self.fm.header_size)))?
//                 - self.fm.header_size as uPS,
//         )
//     }
//     pub fn set_cursor_relative(&mut self, pos: iPS) -> Result<uPS> {
//         Ok(Self::err_tunnel(
//             self.ptr
//                 .seek(SeekFrom::Current(pos + self.fm.header_size as iPS)),
//         )? - self.fm.header_size as uPS)
//     }
//     pub fn read(&mut self, buf: &mut [u8]) -> Result<()> {
//         Self::err_tunnel(self.ptr.read_exact(buf))
//     }
//     pub fn read_cursoring(&mut self, buf: &mut [u8], pos: uPS) -> Result<()> {
//         self.set_cursor(pos)?;
//         Self::err_tunnel(self.ptr.read_exact(buf))
//     }
//     pub fn read_general(&mut self, buf: &mut [u8]) -> Result<LS> {
//         Self::err_tunnel(self.ptr.read(buf))
//     }
//     /// realtime
//     pub fn content_end_pos(&mut self) -> Result<uPS> {
//         Ok(Self::err_tunnel(self.ptr.seek(SeekFrom::End(0)))? - self.fm.header_size)
//     }
//     fn err_tunnel<E>(io_e: std::io::Result<E>) -> Result<E> {
//         errors::handle_io_error(io_e)
//     }
//     pub fn debug(&mut self) -> Result<()> {
//         let mut buf = [0u8; 256];
//         let mut num_read;
//         let prev_pos = SeekFrom::Current(0);
//         self.set_cursor(0)?;
//         loop {
//             num_read = self.read_general(&mut buf[..])?;
//             if num_read < 1 {
//                 break;
//             }
//             eprint!("{:?}", &buf[..num_read]);
//         }
//         eprintln!();
//         self.ptr.seek(prev_pos)?;
//         Ok(())
//     }
// }
