/*
    .. + mod.rs + ..

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

pub mod args;

pub mod envs;
pub use envs::{verbose, Config, Envs};

mod term;
pub use term::{cat, style, Key, OtherKeys, SelItem, Style, Term, TermPassword};

use crate::*;
use hdkey::WrappedKeypair;
use outter_conf::OutterConfig;

pub fn base_loop(
    term: &mut Term,
    _conf: OutterConfig,
    _master_key: WrappedKeypair,
    sl: &mut ServerList,
) -> Result<()> {
    let args_inner = &mut args::inner::new();

    let (tx, _rx) = unbounded::<()>(); // for cli sub thread

    loop {
        term.print_domain();

        let command = term.base_loop(name!(COMMAND));

        let arguments = command.split_whitespace().collect();

        match args_inner.matches(arguments) {
            // -s or --server | print servers statement
            Ok(args) if args.is_present(name!(server: l)) => {
                for s in sl.iter() {
                    let a = term.style(name!(SERVER));
                    let b = term.style(name!(ITALIC_ALERT));
                    term.println(cat!("{} {}", a.apply_to(s.name), b.apply_to("ON")));
                }
            }

            Ok(args) => match args.subcommand() {
                // clear | clear screen of stdout & stderr
                (name!(clear), Some(_)) => term.clear_all(),

                // p | break current specific process
                (name!(p), Some(_)) => tx.try_send(()).unwrap_or_default(),

                // quit | terminate program
                (name!(quit), Some(_)) => break,

                // restart <API/RPC/BOTH> | restart servers
                (name!(restart), Some(m)) => match m.value_of(name!(TARGET)).unwrap_or_default() {
                    name!(API) => print_unwrap!(net::restart(sl, name!(API))),
                    name!(RPC) => print_unwrap!(net::restart(sl, name!(RPC))),
                    name!(BOTH) => print_unwrap!(net::restart(sl, name!(BOTH))),
                    _ => {}
                },

                // stop <API/RPC/BOTH> | stop servers
                (name!(stop), Some(m)) => match m.value_of(name!(TARGET)).unwrap_or_default() {
                    name!(API) => print_unwrap!(net::find_and_stop(sl, name!(API))),
                    name!(RPC) => print_unwrap!(net::find_and_stop(sl, name!(RPC))),
                    name!(BOTH) => net::stop(sl),
                    _ => {}
                },

                // test | testing features
                (name!(test: l), Some(m)) => {
                    term.lock();
                    match m.subcommand() {
                        ("otoodb", Some(mm)) => {
                            let test_mode = mm.is_present("delete");
                            let v = m.occurrences_of(name!(verbose: s)) as u8;
                            test::otoodb(term, test_mode, v)?;
                        }
                        ("wallet", Some(_)) => {}
                        _ => term.eprintln("test --help"),
                    }
                    term.unlock();
                }

                // echo | echo <env_name>
                (name!(echo), Some(m)) => {
                    let env_name = m.value_of(name!(env)).unwrap_or("");
                    term.eprintln(cat!(
                        "{}",
                        std::env::var(env_name).unwrap_or_else(|_| "[None]".to_string())
                    ));
                }

                _ => term.eprintln("* Invalid command"),
            },

            Err(e) => term.eprintln(cat!("{}\n", e)),
        }
    }
    Ok(())
}
