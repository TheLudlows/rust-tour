use std::{fs::File, io::{BufReader, BufRead, Seek}, sync::Arc};

use arrow::{
    csv::{reader::Format, ReaderBuilder},
    datatypes::{DataType, Field, Schema},
};
#[test]
fn test_arr() {
    let s = Schema::empty();

    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    let data = reqwest::blocking::get(url).unwrap().bytes().unwrap().to_vec();
    // println!("{}", String::from_utf8_lossy(&data.to_vec()));
    let mut reader = std::io::Cursor::new(&data);
    let (schema, size) = Format::default()
        .with_header(true)
        
        .infer_schema(&mut reader, Some(0))
        .unwrap();
    let mut reader = std::io::Cursor::new(&data);
    //println!("{}", schema);
    println!("{}", size);
    let schema = Arc::new(schema);
    reader.rewind().unwrap();
    let mut csv = ReaderBuilder::new(schema.clone()).has_header(true).build(&mut reader).unwrap();
    println!("{:?}", schema.all_fields());
    println!("{:?}", schema.metadata());

    let batch = csv.next().unwrap().unwrap();
         println!("{:?}", batch);
    //

    let c = condition::
}
