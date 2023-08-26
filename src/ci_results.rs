use std::collections::HashMap;

#[derive(Debug)]
struct Test {
    name: String,
    passing: bool,
}

impl Test {
    fn new(name: String) -> Test {
        Test {
            name,
            passing: false,
        }
    }
}

#[derive(Debug)]
struct TestFile {
    test_names: Vec<Test>,
}

impl TestFile {
    fn new() -> TestFile {
        TestFile { test_names: vec![] }
    }
}

#[derive(Debug)]
pub struct CIResult {
    tests: HashMap<String, TestFile>,
}

impl CIResult {
    pub fn new() -> CIResult {
        CIResult {
            tests: HashMap::new(),
        }
    }

    pub fn insert(&mut self, file_name: String, test_name: String) {
        let entry = self.tests.entry(file_name).or_insert(TestFile::new());
        let test = Test::new(test_name);
        entry.test_names.push(test);
    }
}
