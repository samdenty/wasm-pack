extern crate assert_cmd;
extern crate failure;
extern crate predicates;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate binary_install;
extern crate serde_json;
#[macro_use]
extern crate serial_test_derive;
extern crate structopt;
extern crate tempfile;
extern crate wasm_pack;

mod build;
mod download;
mod generate;
mod license;
mod lockfile;
mod manifest;
mod readme;
mod stamps;
mod test;
mod utils;
mod wasm_opt;
mod webdriver;
