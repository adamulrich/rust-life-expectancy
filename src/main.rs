
use std::{collections::HashMap};
use std::error::Error;
use serde::Deserialize;
use std::fmt::{self, Display};
use std::clone::Clone;


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

#[derive(Debug, Deserialize, Clone)]
struct HashStruct {
    code: String,
    life_expectancy: f64
}

#[derive(Debug, Deserialize, Clone)]
struct ListStruct {
    region: String,
    code: String,
    year: i32,
    life_expectancy: f64
}

fn main() {

    let QUERY_TYPE: [String; 2] = ["GENERAL".to_string(), "DELTA".to_string()];
    
    // read data from file into dictionary
    let result = read_from_file("res/life-expectancy.csv".to_string()).unwrap();

    // get data from result
    let life_dict = result.0;
    let life_list = result.1 ;

    println!("Done");
}

fn read_from_file(path: String) -> Result<(HashMap<String, HashStruct>, Vec<ListStruct>), Box<dyn Error >> {

    // Creates a new csv `Reader` from a file
    let reader = csv::Reader::from_path(path);
    let mut life_dict: HashMap<String, HashStruct> = HashMap::new();    
    let mut life_list: Vec<ListStruct> = Vec::new();

    
    // `.records` return an iterator of the internal
    // record structure
    for row in reader?.deserialize() {
        let mut record: ListStruct = row?;
        // println!("{}", record.code);
        
        record.code = record.code.to_string().to_lowercase();
        record.region = record.region.to_string().to_lowercase();
    
        // deconstruct
        let year = record.year;
        
        let key = format!("{}{}", record.region, year);

        let value_hash: HashStruct = HashStruct 
            { code: (record.clone().code), 
            life_expectancy: (record.clone().life_expectancy) };
        
        // add to dictionary
        life_dict.insert(key.clone(),value_hash);
      
        // add to list
        life_list.push(record);
    }
    //return (life_dict, life_list);
    Ok((life_dict, life_list))
    
}
