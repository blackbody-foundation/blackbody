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

        let mut new_config = Envs::new_config();

        term.reset_screen();

        term.eprintln("mnemonic language:");
        new_config.hd_lang = term.get_select(&[
            SelItem("english", "en"),
            SelItem("korean", "ko"),
            SelItem("italian", "it"),
            SelItem("french", "fr"),
            SelItem("spanish", "es"),
            SelItem("japanese", "ja"),
            SelItem("chinese-simplified", "zh-hans"),
            SelItem("chinese-traditional", "zh-hant"),
        ]);

        // get dirs

        //"please prepare a pencil for recording your mnemonic."
        //"you will have a little random time, words will be shown four times in total."
        term.reset_screen();
        term.eprintln("successfully created!");

        if reset_mode {
            envs.delete()?;
        }
        let _ = envs.save(&account_password, new_config)?;
        if let Ok(v) = envs.load(&account_password) {
            let lang = match hdkey::Language::from_language_code(v.hd_lang.as_str()) {
                Some(v) => v,
                None => return errbang!(err::BrokenContent, "envs.locked is broken."),
            };
            return Ok(key::master::safe_key(key::master::save_original_key(
                key_password,
                salt,
                lang,
                account_password,
                &v.hd_dirs,
            )?));
        } else {
            something_wrong!(name!(FileIsNotWritten))();
        }
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
    let login_password = out.1;
    let config = out.0;

    if login_password.is_empty() {
        return errbang!(err::MysteriousError, "login password is empty.");
    }

    let keypair = {
        term.reset_screen();
        term.eprintln("master key:");
        let words = term.read_password(true);

        term.reset_screen();
        term.eprintln("key salt:");
        let salt = term.read_password(false).parse::<usize>()?;

        term.reset_screen();
        let lang = match hdkey::Language::from_language_code(config.hd_lang.as_str()) {
            Some(v) => v,
            None => return errbang!(err::BrokenContent, "envs.locked is broken."),
        };

        key::master::safe_key(key::master::read_original_key(
            words,
            salt,
            lang,
            login_password,
            config.hd_dirs.as_slice(),
        )?)
    };

    config.drop(); // because of importance, specify this.
    Ok(keypair)
}
