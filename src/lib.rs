#![allow(warnings)]
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use rand::Rng;

pub fn random_string(l: usize) -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(l)
        .collect()
}
