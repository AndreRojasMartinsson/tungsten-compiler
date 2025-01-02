extern crate string_cache;
pub use atom::*;

#[macro_use]
mod atom;

pub fn guess_host_target_triple() -> String {
    let arch = target::arch();
    let vendor = target::vendor();
    let sys = target::os();
    let env = target::env();

    format!("{arch}-{vendor}-{sys}-{env}")
}
