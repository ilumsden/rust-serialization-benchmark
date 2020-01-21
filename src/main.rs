mod record;
mod types;
mod data;
mod result;

use crate::record::Record;
use crate::data::{STRING_COUNT, STRING_VALUE, INTEGERS};
use crate::result::TestResult;

use std::env;
use std::time::{Duration, Instant};
use avro_rs::{Writer, Reader, Schema, Codec, from_value};

fn add_data_to_record(rec: &mut Record) {
    for _i in 0..STRING_COUNT {
        rec.strings.push(String::from(STRING_VALUE));
    };
    for i in INTEGERS.iter() {
        rec.ids.push(i.clone());
    };
}

fn json_serde_timer(iterations: u64) -> TestResult {
    let mut rec = Record::new();
    add_data_to_record(&mut rec);
    
    let mut buf = serde_json::to_vec(&rec).unwrap();
    {
        let rec2 = serde_json::from_slice(&buf).unwrap();

        if rec != rec2 {
            panic!("Serde_Json failed to serialize and deserialize the record correctly.");
        };
    }

    let start = Instant::now();
    for _i in 0..iterations {
        buf.clear();
        buf = serde_json::to_vec(&rec).unwrap();
        let _rec2: Record = serde_json::from_slice(&buf).unwrap();
    };
    let duration = start.elapsed();
    TestResult::new_str("serde_json", "1.0.44", buf.len(), duration.as_millis())
}

fn cbor_serde_timer(iterations: u64) -> TestResult {
    let mut rec = Record::new();
    add_data_to_record(&mut rec);
    
    let mut buf = serde_cbor::to_vec(&rec).unwrap();
    {
        let rec2 = serde_cbor::from_slice(&buf).unwrap();

        if rec != rec2 {
            panic!("Serde_CBOR failed to serialize and deserialize the record correctly.");
        };
    }

    let start = Instant::now();
    for _i in 0..iterations {
        buf.clear();
        buf = serde_cbor::to_vec(&rec).unwrap();
        let _rec2: Record = serde_cbor::from_slice(&buf).unwrap();
    };
    let duration = start.elapsed();
    TestResult::new_str("serde_cbor", "0.11.1", buf.len(), duration.as_millis())
}

fn msgpack_serde_timer(iterations: u64) -> TestResult {
    let mut rec = Record::new();
    add_data_to_record(&mut rec);
    
    let mut buf = rmp_serde::to_vec(&rec).unwrap();
    {
        let rec2 = rmp_serde::from_slice(&buf).unwrap();

        if rec != rec2 {
            panic!("Serde RMP failed to serialize and deserialize the record correctly.");
        };
    }

    let start = Instant::now();
    for _i in 0..iterations {
        buf.clear();
        buf = rmp_serde::to_vec(&rec).unwrap();
        let _rec2: Record = rmp_serde::from_slice(&buf).unwrap();
    };
    let duration = start.elapsed();
    TestResult::new_str("rmp_serde", "0.14.0", buf.len(), duration.as_millis())
}

fn toml_serde_timer(iterations: u64) -> TestResult {
    let mut rec = Record::new();
    add_data_to_record(&mut rec);
    
    let mut buf = toml::ser::to_vec(&rec).unwrap();
    {
        let rec2 = toml::de::from_slice(&buf).unwrap();

        if rec != rec2 {
            panic!("Serde TOML failed to serialize and deserialize the record correctly.");
        };
    }

    let start = Instant::now();
    for _i in 0..iterations {
        buf.clear();
        buf = toml::ser::to_vec(&rec).unwrap();
        let _rec2: Record = toml::de::from_slice(&buf).unwrap();
    };
    let duration = start.elapsed();
    TestResult::new_str("toml", "0.5.6", buf.len(), duration.as_millis())
}

fn pickle_serde_timer(iterations: u64) -> TestResult {
    let mut rec = Record::new();
    add_data_to_record(&mut rec);
    
    let mut buf = serde_pickle::ser::to_vec(&rec, true).unwrap();
    {
        let rec2 = serde_pickle::de::from_slice(&buf).unwrap();

        if rec != rec2 {
            panic!("Serde TOML failed to serialize and deserialize the record correctly.");
        };
    }

    let start = Instant::now();
    for _i in 0..iterations {
        buf.clear();
        buf = serde_pickle::ser::to_vec(&rec, true).unwrap();
        let _rec2: Record = serde_pickle::de::from_slice(&buf).unwrap();
    };
    let duration = start.elapsed();
    TestResult::new_str("serde_pickle", "0.6.0", buf.len(), duration.as_millis())
}

fn ron_serde_timer(iterations: u64) -> TestResult {
    let mut rec = Record::new();
    add_data_to_record(&mut rec);
    
    let mut buf = ron::ser::to_string(&rec).unwrap();
    {
        let rec2 = ron::de::from_str(&buf).unwrap();

        if rec != rec2 {
            panic!("Serde TOML failed to serialize and deserialize the record correctly.");
        };
    }

    let start = Instant::now();
    for _i in 0..iterations {
        buf.clear();
        buf = ron::ser::to_string(&rec).unwrap();
        let _rec2: Record = ron::de::from_str(&buf).unwrap();
    };
    let duration = start.elapsed();
    TestResult::new_str("ron", "0.5.1", buf.len(), duration.as_millis())
}

fn avro_serde_timer(iterations: u64) -> TestResult {
    let raw_schema = r#"
        {
            "type": "record",
            "name": "test",
            "fields": [
                {"name": "ids", "type": "array", "items": "long"},
                {"name": "strings", "type": "array", "items": "string"}
            ]
        }
    "#;
    let schema = Schema::parse_str(raw_schema).unwrap();
    let mut writer = Writer::with_codec(&schema, Vec::new(), Codec::Deflate);

    let mut rec = Record::new();
    add_data_to_record(&mut rec);

    writer.append_ser(&rec).unwrap();
    writer.flush().unwrap();

    let mut buf = writer.into_inner();
    {
        let reader = Reader::with_schema(&schema, &buf[..]).unwrap();
        let mut recs: Vec<Record> = Vec::new();
        for r in reader {
            recs.push(from_value(&(r.unwrap())).unwrap());
        };
        if rec != recs[0] {
            panic!("Serde Avro failed to serialize and deserialize the record correctly.");
        };
    }

    let start = Instant::now();
    for _i in 0..iterations {
        buf.clear();

        let mut writer = Writer::with_codec(&schema, Vec::new(), Codec::Deflate);
        writer.append_ser(&rec).unwrap();
        writer.flush().unwrap();

        buf = writer.into_inner();
        let reader = Reader::with_schema(&schema, &buf[..]).unwrap();
        let mut recs: Vec<Record> = Vec::new();
        for r in reader {
            recs.push(from_value(&(r.unwrap())).unwrap());
        };
    };
    let duration = start.elapsed();
    TestResult::new_str("avro-rs", "0.6.6", buf.len(), duration.as_millis())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: rust-serializer-benchmark [Num of iterations of each test]");
    };
    let iterations = args[1].parse::<u64>().unwrap();
    let mut res_vec: Vec<TestResult> = Vec::new(); 
    println!("Running JSON test");
    res_vec.push(json_serde_timer(iterations));
    println!("Running CBOR test");
    res_vec.push(cbor_serde_timer(iterations));
    println!("Running msgpack test");
    res_vec.push(msgpack_serde_timer(iterations));
    println!("Running TOML test");
    res_vec.push(toml_serde_timer(iterations));
    println!("Running Pickle test");
    res_vec.push(pickle_serde_timer(iterations));
    println!("Running RON test");
    res_vec.push(ron_serde_timer(iterations));
    println!("Running Avro test");
    res_vec.push(avro_serde_timer(iterations));
    for v in res_vec.iter() {
        println!("Name: {}, Version: {}, Size: {} bytes, Time: {} ms", v.name, v.version, v.size, v.time);
    }
}
