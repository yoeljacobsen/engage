use std::path::Path;
use crate::args::Opts;

fn valid_runnable(name: &str) -> bool {
    return Path::new(name).is_file();
}

pub fn sanity_check(opts: &Opts) {
    assert!(valid_runnable(&opts.command),
            "Provided command isn't reachable: {}",
            &opts.command);
}