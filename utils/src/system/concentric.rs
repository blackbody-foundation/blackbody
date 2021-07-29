/*
    .. + concentric.rs + ..

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

use crate::system::*;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug)]
pub struct Plugin<T>(Arc<Mutex<T>>);

impl<T> Plugin<T> {
    pub fn new(p: T) -> Self {
        Self(Arc::new(Mutex::new(p)))
    }
    pub fn unwrap(&self) -> MutexGuard<T> {
        self.0.lock().unwrap()
    }
}
impl<T> Clone for Plugin<T> {
    fn clone(&self) -> Self {
        Plugin(self.0.clone())
    }
}

pub trait Concentric<T> {
    fn concentric(&mut self, _some_plugin: Option<Plugin<T>>) -> &mut Self;
}

#[macro_export]
macro_rules! employ {
    ($var:expr) => {
        if let Some(p) = &$var {
            Ok(p.unwrap())
        } else {
            errbang!(err::UnwrapingError)
        }
    };
}

pub use employ;
