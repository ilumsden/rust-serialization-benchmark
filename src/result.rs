#[derive(Debug)]
pub struct TestResult {
    pub name: String,
    pub version: String,
    pub size: usize,
    pub time: u128,
}

impl TestResult {
    pub fn new_str(n: &str, v: &str, s: usize, t: u128) -> Self {
        TestResult { name: String::from(n), version: String::from(v), size: s, time: t }
    }
}
