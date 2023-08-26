mod reader;
mod ci_results;
mod util;
mod runner;

use crate::reader::*;
use crate::runner::*;

fn main() {
    let _result = InputBlobReader{}
        .read()
        .and_then(|ci_results|  {
            Runner::new("/home/itarato/Desktop/testserver/testmri", ci_results)
        })
        .map(|mut runner| {
            runner.run();
        });
}
