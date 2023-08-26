use crate::ci_results::*;
use crate::util::*;

use std::path::Path;
use std::process::Command;

pub struct Runner {
    ci_results: CIResult,
}

impl Runner {
    pub fn new<P: AsRef<Path>>(rails_folder: P, ci_results: CIResult) -> Result<Runner, Error> {
        std::env::set_current_dir(rails_folder)?;
        Ok(Runner { ci_results })
    }

    pub fn run(&mut self) {
        let output = Command::new("./bin/rails").arg("test").arg("./test/controllers/posts_controller_test.rb").output();
        dbg!(output);
    }
}
