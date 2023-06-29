
use std::any::Any;
use std::{collections::HashMap};
use std::error::Error;
use serde::Deserialize;
use std::clone::Clone;
use std::io;
use list_comprehension_macro::comp;
use itertools::Itertools;

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

    // let QUERY_TYPE: [String; 2] = ["GENERAL".to_string(), "DELTA".to_string()];
    
    // read data from file into dictionary
    let result = read_from_file("res/life-expectancy.csv".to_string()).unwrap();

    // get data from result
    let life_dict = result.0;
    let life_list = result.1;

    // create dictionary of values
    let region_dict: HashMap<String, String> = create_region_dict(life_list);

    //println!("Done");
    clearscreen::clear().expect("failed to clear screen");

    let another_query = true;

    while another_query {
        // clear screen
        // clearscreen::clear().expect("failed to clear screen");
        println!("Welcome to the Life Expectancy Query System.");
        println!();

        // get query type
        let query_type: i32 = get_int_input(
            "Do you want to do a general age query (1), or a delta query - \
            change in age over a number of years (2)?)".to_string(),
            1,2);


        // get region list from user
        let user_region_list: Vec<String> = get_region_list_from_user(region_dict.clone());

        println!();
        println!("You said {}",user_region_list.join(","));


    }
  
}


// gets an integer from the user within the bounds specified
// will keep trying until successful
fn get_int_input(prompt: String, valid_range_start: i32, valid_range_end: i32) -> i32 {
    // set value outside of range initially.
    let mut input_value = valid_range_end + 1;

    // show prompt
    println!("{}",prompt);

    // while we don't have a good value
    while input_value > valid_range_end || input_value < valid_range_start {

        // read user input
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();

        // convert to integer
        let result: Result<_, _> = user_input.trim().parse::<i32>();

        // handle bad input
        if !result.is_err() {
            // if it is ok, unwrap
            input_value = result.unwrap();
        } 
        // if it is isn't in the range, give error message
        if input_value  < valid_range_start || input_value > valid_range_end  {
            println!("Not valid. Try Again");
        }
    }
    //return value
    input_value
}

fn get_user_input(prompt: String) -> String {
    println!("{}",prompt);
    let mut user_input = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut user_input);
    return user_input;
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


// this function gets a list of regions to query from the user
fn get_region_list_from_user(region_dict: HashMap<String, String>) -> Vec<String> {
    let prompt: String = "What REGIONS(s) would you like to query? \n \
    (You can enter REGION NAME or CODE, separated by commas if more than one region. \n \
    Use ALL for all reagions, or COUNTRIES to exclude continents/regions) ".to_string();

    let mut region_result: Vec<String> = Vec::new();

    // set up while loop flag
    let mut good_region_list = false;

    while !good_region_list {

        // display region list
        let mut x = 0;
        for r in region_dict.keys().sorted() {
            
            print!("{:<8} {:<38}", region_dict[r], r);
            x += 1;
            if x % 4 == 0 {
                println!();
            }
        }
        println!();
        println!();

        let mut bad_code_flag = false;
        let mut region_input_temp: Vec<String> = Vec::new();
        let mut region_output: Vec<String> = Vec::new();
        let mut bad_code_list: Vec<String> = Vec::new();

        // get the user input
        let region_input = get_user_input(prompt.clone());

        //split, lowercase, trim, sort and dedup
        let region_list = region_input.split(",");
        let mut region_input_cleaned: Vec<String> = comp!(r.trim().to_lowercase() for r in region_list);
        region_input_cleaned.dedup();
        
        // handle all case or all countries
        if region_input_cleaned[0] == "all".to_string() {
            
            for r in &region_dict {
                let region = r.0.clone();
                region_input_temp.push(region)
            }
            region_input_cleaned = region_input_temp;
        }

        // if countries, remove the geographic regions.
        if region_input_cleaned[0] == "countries" {
            let remove_list = vec!["Oceania".to_string(),
                                    "Americas".to_string(), 
                                    "Northern America".to_string(),
                                    "Asia".to_string(),
                                    "Europe".to_string(),
                                    "World".to_string(),
                                    "Latin America and the Caribbean".to_string(),
                                    "Africa".to_string()];

            region_input_cleaned.retain(|r| remove_list.contains(r));
        } else {
    
            // iterate over the list
            for r in region_input_cleaned {
                let mut flag_added = false;

                if region_dict.contains_key(&r) || region_dict.values().any(|val| *val == r) {
                    region_output.push(r.clone());
                    flag_added = true;
                }
                // it's not a known code
                else {
                    bad_code_list.push(r.clone());
                    bad_code_flag = true;
                }
                
            }
        }
        // if there are items and the bad flag isn't set, then we can exit
        if region_output.len() > 0 && bad_code_flag == false {
            region_result = region_output;
            good_region_list = true;
        } else {
            println!("Unknown regions in list {}. Please try again.", bad_code_list.join(","));
        }
    
    }
    region_result

}


// create hashmap of codes with  country names
fn create_region_dict(life_list: Vec<ListStruct>) -> HashMap<String, String>{
    
    // create dictionary
    let mut region_dict: HashMap<String, String> = HashMap::new();

    // iterate over vector, create items
    for i in life_list {
        if !region_dict.contains_key(&i.region) {
            region_dict.insert(i.region, i.code);
        }
    }

    region_dict

}