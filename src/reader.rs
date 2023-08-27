use regex::Regex;

use crate::ci_results::*;
use crate::util::*;
use std::io::Read;

/** Sample:

The braced format is useful for exerting precise control over the name of the capture reference.
dev test components/user/test/service_test.rb -n test#some_user_is__ok
For example, ${1}a corresponds to the capture group reference 1 followed by the letter a,
dev test components/login/test/login_test.rb -n test#log_in_with_guest
where as $1a (as mentioned above) corresponds to the capture group reference 1a.

*/

pub trait BlobReader {
    fn read(&self) -> Result<CIResult, Error>;
}

pub struct InputBlobReader;

impl BlobReader for InputBlobReader {
    fn read(&self) -> Result<CIResult, Error> {
        let mut buf = vec![];

        println!("Insert the CI output and press ^d");

        let _read_len = std::io::stdin().read_to_end(&mut buf)?;
        let raw_str = String::from_utf8(buf)?;

        let mut ci_result = CIResult::new();

        let re = Regex::new(r"dev test ([^ ]+) -n ([^ ]+)")?;
        for line in raw_str.lines() {
            let _captures = re.captures(line).map(|capture| {
                ci_result.insert(capture[1].into(), capture[2].into());
            });
        }

        Ok(ci_result)
    }
}
