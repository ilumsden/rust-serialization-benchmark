use serde::{Deserialize, Serialize};
use crate::types::{Integers, Strings};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Record {
    pub ids: Integers,
    pub strings: Strings,
}

impl Record {
    pub fn new() -> Self {
        Record { ids: Vec::<i64>::new(), strings: Vec::<String>::new() }
    }
}
