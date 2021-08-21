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

pub fn login(term: &mut Term) -> Result<hdkey::WrappedKeypair> {
    let envs = Envs::new();

    let mut config = Envs::new_config();
    let mut login_password = String::new();

    // if envs.locked file exists
    if envs.exists() {
        let mut ok = false;

        for _ in 0..MAX_OPPORTUNITY {
            let password = term.read_password();
            login_password = password.clone();
            if let Ok(v) = envs.load(password) {
                config = v;
                config.readed();
                ok = true;
                break;
            }
            term.eprintln(name!(NotMatching));
        }
        if !ok {
            something_wrong!(name!(ForgotPassword))();
        }
    } else {
        // create account
        let password = term.read_password();
        login_password = password.clone();
        let new_config = Envs::new_config();

        term.reset_screen();
        //"please prepare a pencil for recording your mnemonic."
        //"you will have a little random time, words will be shown four times in total."
        term.reset_screen();

        let _ = envs.save(&password, new_config)?;
        if let Ok(v) = envs.load(password) {
            config = v;
            config.readed();
        } else {
            something_wrong!(name!(FileIsNotWritten))();
        }
    };
    // ↑ - get the config structure.
    if config.is_empty() {
        return errbang!(
            err::MysteriousError,
            "cannot load and save envs.locked file."
        );
    }
    if login_password.is_empty() {
        return errbang!(err::MysteriousError, "login password is empty.");
    }

    let keypair = {
        term.reset_screen();
        let words = term.read_password();

        term.reset_screen();
        let salt = term.read_password().parse::<usize>()?;

        term.reset_screen();
        let lang = match hdkey::Language::from_language_code(config.one_shot.lang.as_str()) {
            Some(v) => v,
            None => return errbang!(err::BrokenContent, "envs.locked is broken."),
        };

        key::master::safe_key(key::master::read_original_key(
            words,
            salt,
            lang,
            login_password,
            config.one_shot.hd_dirs.as_slice(),
        )?)
    };

    config.drop(); // because of importance, specify this.
    Ok(keypair)
}
