use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

/// ## Custom Clap App
pub struct CApp<'a, 'b>(App<'a, 'b>);

impl<'a, 'b> CApp<'a, 'b> {
    pub fn new() -> Self {
        Self(
            App::new(crate_name!())
                .version(crate_version!())
                .author(crate_authors!())
                .about(crate_description!()),
        )
    }
    pub fn push(self, arg: Arg<'a, 'b>) -> Self {
        Self(self.0.arg(arg))
    }
    pub fn sink(self) -> App<'a, 'b> {
        self.0
    }
}
