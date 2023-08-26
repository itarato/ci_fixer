use crate::ci_results::*;
use crate::util::*;

use std::path::Path;
use std::process::Command;

const RAILS_BIN: &'static str = "./bin/rails";
const CMD_TEST: &'static str = "test";

pub struct Runner {
    ci_results: CIResult,
}

impl Runner {
    pub fn new<P: AsRef<Path>>(rails_folder: P, ci_results: CIResult) -> Result<Runner, Error> {
        std::env::set_current_dir(rails_folder)?;
        Ok(Runner { ci_results })
    }

    pub fn run(&mut self) {
        let output = Command::new(RAILS_BIN)
            .arg(CMD_TEST)
            .arg("./test/controllers/posts_controller_test.rb")
            .output();

        dbg!(output);
    }
}
