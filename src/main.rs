mod cmd;
mod entities;
mod macros;
mod services;

use ::clap::Parser;

use crate::cmd::CMD;

#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod test_utils;

fn main() {
  let _ = CMD::parse();
}
