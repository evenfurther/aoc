#![allow(clippy::type_complexity)]

use lazy_static::lazy_static;
use std::{collections::BTreeMap, fmt::Display, sync::Mutex};

type Runner = dyn FnOnce() -> eyre::Result<String> + Send + Sync + 'static;

lazy_static! {
    pub(crate) static ref RUNNERS: Mutex<BTreeMap<(usize, usize), Vec<(Option<String>, Box<Runner>)>>> =
        Mutex::new(BTreeMap::new());
}

pub fn register_runner<F, T>(day: usize, part: usize, version: Option<String>, func: F)
where
    F: FnOnce() -> eyre::Result<T> + Send + Sync + 'static,
    T: Display,
{
    let mut map = RUNNERS.lock().unwrap();
    map.entry((day, part))
        .or_default()
        .push((version, Box::new(|| func().map(|r| r.to_string()))));
}
