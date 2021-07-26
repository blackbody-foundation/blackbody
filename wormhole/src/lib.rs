/*
    .. + lib.rs + ..

    Copyright (C) 2021 Hwakyeom Kim(=just-do-halee)

    BlackBody is free software: you can redistribute it and/or modify
    it under the terms of the GNU Lesser General Public License as
    published by the Free Software Foundation, either version 3 of the
    License, or (at your option) any later version.

    BlackBody is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License
    along with BlackBody. If not, see <http://www.gnu.org/licenses/>.

*/

mod cmn;
use cmn::*;

mod tgroup; // thread group
use tgroup as tg;

mod cccs;
pub use cccs::head::CCCSHeader;

///```no_run
/// {
///     pub db_path: PathBuf,
///     pub src_atom_len: LS,
///     pub dst_atom_len: LS,
/// }
///```
/// this has only Path and Sizes(two of usize) so it's cheap to clone or new
pub struct Wormhole {
    pub db_path: PathBuf,
    pub src_atom_len: LS,
    pub dst_atom_len: LS,
}

impl Wormhole {
    pub fn new(db_path: &str, src_atom_len: LS, dst_atom_len: LS) -> Self {
        Self {
            db_path: pathy!(db_path),
            src_atom_len,
            dst_atom_len,
        }
    }
    /// if infile is empty then excute from io::stdin(), out to io::stdout()
    pub fn transform(&self, infile: String) -> Result<()> {
        let db = self.load_otoodb()?;
        tg::TransformTG::new(tg::Requirement::new(infile, db)).join()?;
        Ok(())
    }
    fn load_otoodb(&self) -> Result<DB> {
        DB::open(
            valid_path!(self.db_path)?,
            self.src_atom_len,
            self.dst_atom_len,
        )
    }
}
