
use std::{collections::HashMap};
use std::error::Error;
use serde::Deserialize;
use std::fmt::{self, Display};


// CONSTANTS
static REGION: i32 = 0;
static CODE: i32 = 1;
static YEAR: i32 = 2;
static LIFE_EXPECTANCY: i32 = 3;
static DELTA_YEAR: i32 = 4;
static DELTA_YEAR_LIFE_EXPECTANCY: i32 = 5;
static DELTA: i32 = 6;
static DICT_LIFE_EXPECTANCY: i32 = 1;
static DICT_CODE: i32 = 0;
static QUERY_TYPE_GENERAL: i32 = 0;
static QUERY_TYPE_DELTA: i32 = 1;

enum ListValue {
    Int(i32),
    Float(f64),
    Text(String),
}

#[derive(Debug, Deserialize)]
struct DataPoint {
    region: String,
    code: String,
    year: i32,
    life_expectancy: f64
}

fn main() {

    let QUERY_TYPE: [String; 2] = ["GENERAL".to_string(), "DELTA".to_string()];
    

    // read data from file into dictionary

    let result = read_from_file("res/life-expectancy.csv".to_string()).unwrap();
    
    let life_dict = result.0;
    let life_list = result.1 ;
}


fn read_from_file(path: String) -> Result<(HashMap<String, Vec<ListValue>>, Vec<Vec<ListValue>>), Box<dyn Error >> {

    //(HashMap<String, Vec<ListValue>>, Vec<Vec<ListValue>>)

        // Creates a new csv `Reader` from a file
    let reader = csv::Reader::from_path(path);
    let mut life_dict: HashMap<String, Vec<ListValue>> = HashMap::new();    
    let mut life_list: Vec<Vec<ListValue>> = Vec::new();

    
    // `.records` return an iterator of the internal
    // record structure
    for row in reader?.deserialize() {
        let record: DataPoint = row?;
        println!("{}", record.code);
        
        let region = record.region.to_string().to_lowercase();
        let year = record.year;
        let code = record.code.to_string().to_lowercase();
        let life_expectancy = record.life_expectancy;
        let key = format!("{}{}", region, year);

        println!("{}", region);

        let value_hash = vec![
            ListValue::Text(record.code),
            ListValue::Float(record.life_expectancy)
        ];
        
        life_dict.insert(key.clone(),value_hash);

        
        let value_list = vec![
            ListValue::Text(region),
            ListValue::Text(code),
            ListValue::Int(year),
            ListValue::Float(life_expectancy)
        ];

        life_list.push(value_list);

    
    }
    //return (life_dict, life_list);
    Ok((life_dict, life_list))

    
    
}