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

    pub fn new(n: &str, v: u64, s: usize, t: u128) -> Self {
        TestResult { name: String::from(n), version: v.to_string(), size: s, time: t }
    }
}
