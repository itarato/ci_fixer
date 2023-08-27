mod ci_results;
mod reader;
mod runner;
mod util;

use crate::reader::*;
use crate::runner::*;

fn main() {
    let _result = InputBlobReader {}
        .read()
        .and_then(|ci_results| Runner::new("/home/itarato/Desktop/testserver/testmri", ci_results))
        .and_then(|mut runner| runner.run())
        .unwrap_or_else(|err| {
            eprintln!("Execution has failed with: {}", err);
        });
}
