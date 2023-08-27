use getch::Getch;

use crate::ci_results::*;
use crate::util::*;

use std::io::Write;
use std::path::Path;
use std::process::Command;

const RAILS_BIN: &'static str = "./bin/rails";
const CMD_TEST: &'static str = "test";

#[derive(Debug)]
enum Instruction {
    NextTest,
    NextFile,
    ThisFile,
    ThisTest,
    Abort,
}

pub struct Runner {
    ci_results: CIResult,
}

impl Runner {
    pub fn new<P: AsRef<Path>>(rails_folder: P, ci_results: CIResult) -> Result<Runner, Error> {
        std::env::set_current_dir(rails_folder)?;
        Ok(Runner { ci_results })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        for (file_name, tests) in &mut self.ci_results.tests {
            for test in &mut tests.test_names {
                let mut skip_all_tests_in_file = false;
                loop {
                    match Runner::read_instruction() {
                        Err(_) => continue,
                        Ok(instruction) => match instruction {
                            Instruction::ThisTest => {
                                let output = Command::new(RAILS_BIN)
                                    .arg(CMD_TEST)
                                    .arg(file_name)
                                    .arg("-n")
                                    .arg(&test.name)
                                    .output()?;

                                let status_code = output.status.code().unwrap_or(1);
                                println!("Output: #{}", String::from_utf8(output.stdout)?);

                                if status_code == 0 {
                                    break;
                                } else {
                                    continue;
                                }
                            }
                            Instruction::ThisFile => {
                                let output = Command::new(RAILS_BIN)
                                    .arg(CMD_TEST)
                                    .arg(file_name)
                                    .output()?;

                                let status_code = output.status.code().unwrap_or(1);
                                println!("Output: #{}", String::from_utf8(output.stdout)?);

                                if status_code == 0 {
                                    skip_all_tests_in_file = true;
                                    break;
                                } else {
                                    continue;
                                }
                            }
                            Instruction::NextTest => break,
                            Instruction::NextFile => {
                                skip_all_tests_in_file = true;
                                break;
                            }
                            Instruction::Abort => return Ok(()),
                        },
                    };
                }

                if skip_all_tests_in_file {
                    break;
                }
            }
        }

        Ok(())
    }

    fn read_instruction() -> Result<Instruction, Error> {
        print!("(1) This test (2) this file (3) next test (4) next file (5) abort: ");
        std::io::stdout().flush()?;

        let getch = Getch::new();
        match getch.getch()? {
            b'1' => Ok(Instruction::ThisTest),
            b'2' => Ok(Instruction::ThisFile),
            b'3' => Ok(Instruction::NextTest),
            b'4' => Ok(Instruction::NextFile),
            b'5' => Ok(Instruction::Abort),
            _ => Err("Invalid command".into()),
        }
    }
}
