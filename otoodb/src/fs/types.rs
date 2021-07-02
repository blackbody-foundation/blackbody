use super::header;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

pub type IOFile = File;
pub type Header = Box<dyn header::Header>;
pub type Reader = Box<BufReader<File>>;
pub type Writer = Box<BufWriter<File>>;

pub trait Convert {
    fn into_writer(self) -> Box<BufWriter<File>>;
    fn into_reader(self) -> Box<BufReader<File>>;
}
impl Convert for File {
    fn into_reader(self) -> Box<BufReader<File>> {
        Box::new(BufReader::new(self))
    }
    fn into_writer(self) -> Box<BufWriter<File>> {
        Box::new(BufWriter::new(self))
    }
}
