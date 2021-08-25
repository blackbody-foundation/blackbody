/*
    .. + login.rs + ..

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

use crate::*;

const MAX_OPPORTUNITY: u8 = 3;

pub fn login(term: &mut Term, reset_mode: bool) -> Result<hdkey::WrappedKeypair> {
    let envs = Envs::new();

    term.reset_screen();

    let password_option = TermPassword::new()
        .encrypt(true)
        .max_opportunity(MAX_OPPORTUNITY);

    let mut salt_option = TermPassword::new()
        .encrypt(false)
        .max_opportunity(MAX_OPPORTUNITY)
        .min_length(1)
        .max_length(10);
    salt_option
        .min_length
        .set_error_message(Some("salt must be more than 10"));
    salt_option
        .max_length
        .set_error_message(Some("salt must be less than 999999999"));

    // if envs.locked file not exists
    if !envs.exists() || reset_mode {
        // create account
        term.eprintln("new account:");
        let account_password = term.read_password_op(&password_option, |pass| (true, pass));
        term.eprintln("confirmation:");
        let account_password = term.read_password_op(&password_option, |re_password| {
            if account_password == re_password {
                (true, re_password)
            } else {
                (false, re_password)
            }
        });

        term.reset_screen();
        term.eprintln("new master key:");
        let key_password = term.read_password_op(&password_option, |pass| (true, pass));
        term.eprintln("confirmation:");
        let key_password = term.read_password_op(&password_option, |re_password| {
            if key_password == re_password {
                (true, re_password)
            } else {
                (false, re_password)
            }
        });

        term.reset_screen();
        term.eprintln("new key salt:");
        let salt = term.read_password_op(&salt_option, |password| {
            if let Ok(v) = password.parse::<usize>() {
                (v > 1, v)
            } else {
                (false, 0)
            }
        });
        term.eprintln("confirmation:");
        let salt = term.read_password_op(&salt_option, |password| {
            if let Ok(v) = password.parse::<usize>() {
                (v == salt, v)
            } else {
                (false, 0)
            }
        });

        term.reset_screen();

        term.eprintln("mnemonic language:");
        let lang = term.get_select(&[
            SelItem("english", "en"),
            SelItem("korean", "ko"),
            SelItem("italian", "it"),
            SelItem("french", "fr"),
            SelItem("spanish", "es"),
            SelItem("japanese", "ja"),
            SelItem("chinese-simplified", "zh-hans"),
            SelItem("chinese-traditional", "zh-hant"),
        ]);

        let hd_lang = match hdkey::Language::from_language_code(lang.as_str()) {
            Some(v) => v,
            None => return errbang!(err::BrokenContent, "envs.locked is broken."),
        };

        term.reset_screen();
        let mut dirs;
        let keypair = loop {
            term.eprintln("key saving target directory:\n");
            term.eprintln("number of split folders(range 2 ~ 255):");

            let n_dirs = match term.read_line().parse::<u8>() {
                Ok(v) if v > 1 => v,
                _ => {
                    term.eprintln("range: 2 ~ 255.");
                    continue;
                }
            };

            dirs = Vec::new();
            for _ in 0..n_dirs {
                let mut path;
                loop {
                    term.eprintln("path:");
                    path = term.read_line();
                    if PathBuf::from(&path).exists() {
                        break;
                    } else {
                        term.eprintln("target directory doesn't exist.");
                    }
                }
                dirs.push(path);
            }

            match key::master::save_original_key(
                &key_password,
                salt,
                hd_lang,
                &account_password,
                &dirs,
            ) {
                Ok(v) => break v,
                Err(e) if errmatch!(e, key::master::ShieldPathError) => {
                    term.reset_screen();
                    term.eprintln(cat!("{}\n", e))
                }
                Err(e) => return Err(e),
            }
        };
        let mut new_config = Envs::new_config();
        new_config.new_key(lang, dirs);
        //"please prepare a pencil for recording your mnemonic."
        //"you will have a little random time, words will be shown four times in total."
        term.reset_screen();
        term.eprintln("successfully created!");

        if reset_mode {
            envs.delete()?; // delete envs.locked
        }
        // save envs.locked
        let _ = envs.save(&account_password, new_config)?;
        // load envs.locked
        if let Ok(v) = envs.load(&account_password) {
            // re-load master key
            let keypair_reload = key::master::read_original_key(
                key_password,
                salt,
                hd_lang,
                account_password,
                v.keys[0].dirs.as_slice(),
            )?;
            // last check
            if keypair == keypair_reload {
                return Ok(key::master::safe_key(keypair)); // return master keypair
            } else {
                something_wrong!(name!(FileIsNotWritten))();
            }
        } else {
            something_wrong!(name!(FileIsNotWritten))();
        }
    };

    // if key already exists

    // select account
    let n_account = 0;

    term.eprintln("account:");
    let out = term.read_password_op(&password_option, |password| {
        if let Ok(conf) = envs.load(&password) {
            (true, (conf, password))
        } else {
            (false, (Envs::new_config(), password))
        }
    });

    // extract things
    let login_password = out.1;
    let config = out.0;

    if login_password.is_empty() {
        return errbang!(err::MysteriousError, "login password is empty.");
    }

    let keypair = {
        term.reset_screen();
        term.eprintln("master key:");
        let words = term.read_password_op(&password_option, |pass| (true, pass));

        term.reset_screen();
        term.eprintln("key salt:");
        let salt = term.read_password_op(&salt_option, |password| {
            if let Ok(v) = password.parse::<usize>() {
                (v > 1, v)
            } else {
                (false, 0)
            }
        });

        term.reset_screen();
        let lang = match hdkey::Language::from_language_code(config.keys[n_account].lang.as_str()) {
            Some(v) => v,
            None => return errbang!(err::BrokenContent, "envs.locked is broken."),
        };

        key::master::safe_key(errextract!(key::master::read_original_key(
                words,
                salt,
                lang,
                login_password,
                config.keys[n_account].dirs.as_slice(),
            ),
            key::master::ShieldPathNotMatching => something_wrong!("login failed")()))
    };

    config.drop(); // because of importance, specify this.
    Ok(keypair)
}
