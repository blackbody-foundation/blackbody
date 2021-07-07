use crate::macros::errbang::*;

errors! {
    BrokenHeader => "broken header.",
    AnotherHeader => "not matched header.",
    FileNotFound => "file not found.",
}
