/*
    .. + main.rs + ..

    Copyright 2021 Hwakyeom Kim(=just-do-halee)

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

mod cli;

mod net;

mod key;
use key::Version;

pub const VERSION: Version = Version::TestNet; // current version

mod outter_conf;
mod test;

fn main() -> Result<()> {
    let term = &mut Term::new();
    term.init();

    let conf = outter_conf::get();
    let master_key = key::login(term, conf.reset_mode)?;
    let sl = &mut net::run(&conf.net_mode)?;

    cli::base_loop(term, conf, master_key, sl)?;
    net::stop(sl);

    Ok(())
}
