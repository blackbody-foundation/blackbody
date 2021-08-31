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
        .max_length(10)
        .not_matching_message("salt (10 ~ 999999999)");
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
        return create_new_master_key(
            term,
            account_password,
            &password_option,
            &salt_option,
            envs,
            None,
            reset_mode,
        );
    };

    // if key already exists
    term.eprintln("account:");
    let out = term.read_password_op(&password_option, |password| {
        if let Ok(conf) = envs.load(&password) {
            (true, (conf, password))
        } else {
            (false, (Envs::new_config(), password))
        }
    });

    // extract things
    let account_password = out.1;
    let mut config = out.0;

    if account_password.is_empty() {
        return errbang!(err::MysteriousError, "login password is empty.");
    }

    // empty key
    if config.keys.is_empty() {
        // new key
        return create_new_master_key(
            term,
            account_password,
            &password_option,
            &salt_option,
            envs,
            Some(config),
            false,
        );
    }

    let n_key = get_select_n_key(term, &config); // select key

    let (n_key, remove_mode) = if n_key == "new" {
        // if user select 'n' key for new master key
        // new key
        return create_new_master_key(
            term,
            account_password,
            &password_option,
            &salt_option,
            envs,
            Some(config),
            false,
        );
    } else if n_key.ends_with(" remove") {
        ((&n_key[0..n_key.len() - 7]).parse::<usize>()?, true)
    } else {
        (n_key.parse::<usize>()?, false)
    };

    let keypair = {
        term.reset_screen();
        term.eprintln("key password:");
        let words = term.read_password_op(&password_option, |pass| (true, pass));

        term.reset_screen();
        term.eprintln("key salt:");
        let salt = term.read_password_op(&salt_option, |password| {
            if let Ok(v) = password.parse::<usize>() {
                (v > 9, v)
            } else {
                (false, 0)
            }
        });

        term.reset_screen();
        let lang = match hdkey::Language::from_language_code(config.keys[n_key].lang.as_str()) {
            Some(v) => v,
            None => return errbang!(err::BrokenContent, "envs.locked is broken."),
        };
        let (keypair, mnemonic) = errextract!(key::master::read_original_key(
                Password::new(&words)?,
                salt,
                lang,
                Password::new(&account_password)?,
                config.keys[n_key].dirs.as_slice(),
            ),
            key::master::ShieldPathNotMatching => something_wrong!("login failed")());

        if remove_mode {
            term.eprintln("really do you want remove the key?  (y/n)\n");
            extract_key!(term,
                Key::Char('y') => {break},
                Key::Char('n') => {
                    something_wrong!("please restart blackbody.")();
                },
                _ => {continue}
            );
            term.reset_screen();
            term.eprintln(
                "then please input your first five mnemonic words. (seprate whitespace)\n",
            );
            let check = loop {
                let check = term.read_line();
                if !check.is_empty() {
                    break check;
                }
            };
            if check.as_str()
                == mnemonic
                    .split_whitespace()
                    .take(5)
                    .collect::<Vec<&str>>()
                    .join(" ")
            {
                key::master::remove_original_key(
                    Password::new(&words)?,
                    salt,
                    Password::new(&account_password)?,
                    config.keys[n_key].dirs.as_slice(),
                )?;
                config.remove_key(n_key);
                envs.save(&account_password, config)?;
                drop(words);
                drop(account_password);
                term.eprintln("removed!");
                thread::sleep(Duration::from_secs(2));
                return login(term, false);
            } else {
                something_wrong!("failed!")();
            }
        } else {
            drop(account_password);
        }
        key::master::safe_key(keypair)
    };

    config.drop(); // because of importance, specify this.
    Ok(keypair)
}

fn get_select_n_key(term: &mut Term, config: &Config) -> String {
    term.reset_screen();
    term.eprintln("master key:");

    let key_count = config.keys.len();

    let mut sel_list = Vec::with_capacity(key_count);

    let index = (0..key_count)
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    for (i, key) in config.keys.iter().enumerate() {
        let len = key.address.len();
        sel_list.push(SelItem(&key.address[..len - 16], &index[i]));
    }

    term.get_select(
        &sel_list,
        Some((
            &[" [ n ]   new key", " [ r ]   remove key"],
            Box::new(|c, mut res| match c {
                Key::Char('n') => (true, String::from("new")),
                Key::Char('r') => {
                    res.push_str(" remove");
                    (true, res)
                }
                _ => (false, res),
            }),
        )),
    )
}

fn show_mnemonic(term: &mut Term, mnemonic: String) {
    term.reset_screen();
    term.eprintln("please prepare a pencil for recording your mnemonic.\n");
    thread::sleep(Duration::from_secs(1));
    term.eprintln("the words will be shown four times in a total.\n");
    thread::sleep(Duration::from_secs(1));
    term.eprintln("if you are ready, press any key.\n");
    let _ = term.read_key();
    term.reset_screen();

    let mnemonic = mnemonic.split_whitespace().collect::<Vec<&str>>();
    let size = mnemonic.len() / 4;
    term.hide_cursor();
    term.reset_screen();
    for (i, chunk) in mnemonic.chunks(size).enumerate() {
        term.eprint(cat!(
            "\r{}.\n\n{}\n\npress any key..",
            i + 1,
            chunk.join(" ")
        ));
        term.move_cursor_up(4);
        let _ = term.read_key();
    }
    term.move_cursor_down(1);
    term.clear_line();
    term.reset_screen();
    term.show_cursor();
}

fn create_new_master_key(
    term: &mut Term,
    account_password: String,
    password_option: &TermPassword,
    salt_option: &TermPassword,
    envs: Envs,
    config: Option<Config>,
    reset_mode: bool,
) -> Result<hdkey::WrappedKeypair> {
    term.reset_screen();
    term.eprintln("new master key:");
    let key_password = term.read_password_op(password_option, |pass| (true, pass));
    term.eprintln("confirmation:");
    let key_password = term.read_password_op(password_option, |re_password| {
        if key_password == re_password {
            (true, re_password)
        } else {
            (false, re_password)
        }
    });

    term.reset_screen();
    term.eprintln("new key salt:");
    let salt = term.read_password_op(salt_option, |password| {
        if let Ok(v) = password.parse::<usize>() {
            (v > 9, v)
        } else {
            (false, 0)
        }
    });
    term.eprintln("confirmation:");
    let salt = term.read_password_op(salt_option, |password| {
        if let Ok(v) = password.parse::<usize>() {
            (v == salt, v)
        } else {
            (false, 0)
        }
    });

    term.reset_screen();

    term.eprintln("mnemonic language:");
    let lang = term.get_select(
        &[
            SelItem("english", "en"),
            SelItem("korean", "ko"),
            SelItem("italian", "it"),
            SelItem("french", "fr"),
            SelItem("spanish", "es"),
            SelItem("japanese", "ja"),
            SelItem("chinese-simplified", "zh-hans"),
            SelItem("chinese-traditional", "zh-hant"),
        ],
        None,
    );

    let hd_lang = match hdkey::Language::from_language_code(lang.as_str()) {
        Some(v) => v,
        None => return errbang!(err::BrokenContent, "envs.locked is broken."),
    };

    term.reset_screen();
    let mut dirs;
    let (keypair, mnemonic) = loop {
        term.eprintln("key saving target directory:\n");
        term.eprintln("number of split folders(range 2 ~ 255):");

        let n_dirs = match term.read_line().parse::<u8>() {
            Ok(v) if v > 1 => v,
            _ => {
                term.eprintln("range: 2 ~ 255.");
                continue;
            }
        };

        dirs = Vec::with_capacity(n_dirs as usize);
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

        term.reset_screen();
        match key::master::save_original_key(
            Password::new(&key_password)?,
            salt,
            hd_lang,
            Password::new(&account_password)?,
            &dirs,
        ) {
            Ok(v) => break v,
            Err(e) if errmatch!(e, key::master::ShieldPathError) => term.eprintln(cat!("{}\n", e)),
            Err(e) => return Err(e),
        }
    };

    show_mnemonic(term, mnemonic);

    let mut new_config = if let Some(conf) = config {
        conf
    } else {
        Envs::new_config()
    };
    new_config.new_key(hex::encode(keypair.public()), lang, dirs);

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
        let last_key_index = v.keys.len() - 1;
        let (keypair_reload, _) = key::master::read_original_key(
            Password::new(key_password)?,
            salt,
            hd_lang,
            Password::new(account_password)?,
            v.keys[last_key_index].dirs.as_slice(),
        )?;
        // last check
        if keypair == keypair_reload {
            Ok(key::master::safe_key(keypair)) // return master keypair
        } else {
            something_wrong!(name!(FileIsNotWritten))()
        }
    } else {
        something_wrong!(name!(FileIsNotWritten))()
    }
}
