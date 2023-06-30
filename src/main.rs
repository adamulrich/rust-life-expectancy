

use std::{collections::HashMap};
use std::error::Error;
use serde::Deserialize;
use std::clone::Clone;
use std::io;
use list_comprehension_macro::comp;
use itertools::Itertools;

// CONSTANTS
static QUERY_TYPE_GENERAL: i32 = 0;
static QUERY_TYPE_DELTA: i32 = 1;
static ORDER_LIFE_ASC: i32 = 1;
static ORDER_LIFE_DES: i32 = 2;
static ORDER_DATE_ASC: i32 = 3;
static ORDER_DATE_DES: i32 = 4;
static ORDER_REGION_DES: i32 = 5;
static ORDER_DELTA_ASC: i32 = 6;
static ORDER_DELTA_DES: i32 = 7;


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
    life_expectancy: f64,
    #[serde(skip)]
    delta: f64
}

fn main() {

    let sort_text: Vec<String> = vec!["Life Expectancy: Lowest to Highest".to_string(), 
    "Life Exptectancy: Highest to Lowest".to_string(), 
    "Year: Oldest to Newest".to_string(), 
    "Year: Newest to Oldest".to_string(), 
    "Region: Alphabetical".to_string(), 
    "Delta: Largest to Smallest".to_string(), 
    "Delta: Smallest to Largest".to_string()];
    
    // read data from file into dictionary
    let result = read_from_file("res/life-expectancy.csv".to_string()).unwrap();

    // get data from result
    let life_dict = result.0;
    let life_list = result.1;

    // create dictionary of values
   let region_dict: HashMap<String, String> = create_region_dict(life_list.clone());

    //println!("Done");
    // clearscreen::clear().expect("failed to clear screen");

    let another_query = true;

    while another_query {
        // clear screen
        // clearscreen::clear().expect("failed to clear screen");
        println!("Welcome to the Life Expectancy Query System.");
        println!();

        // get query type
        let mut query_input_type: i32 = get_int_input(
            "Do you want to do a general age query (1), or a delta query - \
            change in age over a number of years (2)?)".to_string(),
            1,2);

        query_input_type -= 1;


        // get region list from user
        let user_region_list: Vec<String> = get_region_list_from_user(region_dict.clone());

        println!();
        println!("You said {}",user_region_list.join(", "));
    
        let mut data_set: Vec<ListStruct> = filter_region(life_list.clone(), user_region_list.clone());

        // get year range for query
        let min_years: i32 = data_set.clone().into_iter().map(|r| r.year).min().unwrap();
        let max_years: i32 = data_set.clone().into_iter().map(|r| r.year).max().unwrap();
        let max_delta_years: i32 = max_years - min_years;

        println!("We have data from {} to {} for this region selection.", min_years,max_years);


        let mut year_delta_input: i32 =0;
        // if delta, get delta value
        if query_input_type == QUERY_TYPE_DELTA {
            let prompt = format!("How many years in the future to get the delta (1 - {max_delta_years}) ? ");
            year_delta_input = get_int_input(prompt, 1, max_delta_years);
        }

        // get year range from user
        let prompt_min = format!("Would year would you like to START your query? ({} - {}) ", min_years, max_years - year_delta_input );
        let start_year_requested = get_int_input(prompt_min, min_years, max_years- year_delta_input);
        let prompt_max = format!("Would year would you like to END your query? ({} - {}) ", min_years, max_years - year_delta_input );
        let end_year_requested = get_int_input(prompt_max, min_years, max_years- year_delta_input);
        let mut order_descending_requested: bool = false;
        let mut order_text: String;
        let mut query_order_input: i32;

        // process general query
        if query_input_type == QUERY_TYPE_GENERAL {
            query_order_input = get_int_input(
                "Would you like the lowest (1) or highest (2) values for life expectancy at the top, \n\
                or would you like it in chronological order oldest to newest (3) or newest to oldest (4), \n\
                or by region name alphabetical (5)? ".to_string(),
                1,5);
            
            order_descending_requested = query_order_input == ORDER_DATE_ASC || 
                                    query_order_input == ORDER_LIFE_ASC;

            order_text = sort_text[query_order_input as usize - 1].to_string();

        } else {
            // it is a delta query

            query_order_input = get_int_input(
                "Would you like the lowest (1) or highest (2) values for life expectancy at the top, \n\
                or would you like it in chronological order oldest to newest (3) or newest to oldest (4), \n\
                or by region name alphabetical (5), or by Delta largest (6) or smallest (7)? ".to_string(),
                1,7);
            
                order_descending_requested = query_order_input == ORDER_DATE_ASC || 
                                    query_order_input == ORDER_LIFE_ASC ||
                                    query_order_input == ORDER_DELTA_DES;
            
            order_text = sort_text[query_order_input as usize - 1].to_string();

        }
        
        // get number of results to display
        let prompt_count = "How many results would you like to display? (1 - 20000) ".to_string();
        let mut results_count_requested = get_int_input(prompt_count, 1, 20000);

        // filter to years
        data_set = filter_years(data_set, start_year_requested, end_year_requested);

        //if there is more than one region, ask if they want to filter to common years
        let mut query_only_common_years = false;
        let user_region_count = user_region_list.clone().len();
        if (user_region_count) > (1 as usize) 
        {
            let prompt_common_years = "You have more than one region. Would you like to filter the dataset to just the years all regions have in common (Y/N)? ".to_string();
            query_only_common_years = get_user_input(prompt_common_years).to_lowercase().chars().next().unwrap() == 'y';
        }
        
        if query_input_type == QUERY_TYPE_DELTA {
            data_set = delta_query(data_set, life_dict.clone(), year_delta_input)
        }

        // sort list by order
        if query_order_input == ORDER_LIFE_DES ||
        query_order_input == ORDER_LIFE_ASC {
                data_set = sort_by_life_expectancy(data_set, order_descending_requested);
            }
        
        if query_order_input == ORDER_DATE_ASC ||
        query_order_input == ORDER_DATE_DES {
                data_set = sort_by_year(data_set, order_descending_requested);
        } 
        
        if query_order_input == ORDER_REGION_DES    
        {
            // sort by region
            data_set = sort_by_region(data_set);
        }

        if query_order_input == ORDER_DELTA_ASC || 
        query_order_input == ORDER_DELTA_DES {
            // sort by delta
            data_set = sort_by_delta(data_set, order_descending_requested)
        }

        // filter to rowcount
        results_count_requested = results_count_requested.min(data_set.len() as i32);
        data_set = data_set[..results_count_requested as usize].to_vec();

        // print data_set for regular
        let query_type: [String; 2] = ["GENERAL".to_string(), "DELTA".to_string()];

        println!();
        println!();
        println!("-------------");
        println!("Query Type: {}", query_type[query_input_type as usize]);
        println!("Sort Type: {}", order_text);
        println!("Row Count: {}", results_count_requested);
        if (user_region_count) > (1 as usize) {
            println!("Common Year Filter: {}", query_only_common_years);
        }
        println!();        

        //get max region width
        let max_width: usize = data_set.clone().into_iter().map(|r| r.region.len()).max().unwrap() + 2;
        
        if query_input_type == QUERY_TYPE_GENERAL {
            println!("{:<8} {:<max_width$} {:<7} {:<8}", "code", "region", "year", "life_exp");
            println!("{:<8} {:<max_width$} {:<7} {:<8}", "----", "------", "----", "---------------");
            for r in data_set {
                println!("{:<8} {:<max_width$} {:<7} {:<8.2}", r.code, r.region, r.year, r.life_expectancy);
            }
        } else {
            // print headers
            println!("{:<8} {:<max_width$} {:<7} {:<8} {:<8}", "code", "region", "year", "life_exp", "delta");
            for r in data_set {
                println!("{:<8} {:<max_width$} {:<7} {:<8.2} {:<8.2}", r.code, r.region, r.year, r.life_expectancy, r.delta);
            }

        }

        println!();
        println!();

        //loop back as needed.
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
    user_input
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
        
        record.code = record.code.to_string().to_lowercase();
        record.region = record.region.to_string().to_lowercase();
        record.delta = 0 as f64;
    
        // deconstruct
        let year = record.year;
        
        let key = format!("{}:{}", record.region, year);

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
    Use ALL for all regions, or COUNTRIES to exclude continents/regions) ".to_string();

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
                let s = r.clone();

                // if it is in the values, get the key and put it in the output
                if region_dict.values().any(|val| *val == r) {
                    let region = &region_dict.iter()
                    .find_map(|(key, val)| if val == &s { Some(key) } else { None }).unwrap();
                    region_output.push(region.to_string());
                } 

                // if it is in the keys, put it in the output
                else if region_dict.contains_key(&r)
                    {
                        region_output.push(r);
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
            bad_code_flag = false;
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
    // return dictionary
    region_dict
}

// filters the dataset to just the requested regions
fn filter_region(data_set: Vec<ListStruct>, user_regions: Vec<String>) -> Vec<ListStruct> {

    data_set.into_iter().filter(|x| user_regions.contains(&x.region)).collect()    
}

// filters to just the years requested
fn filter_years(data_set: Vec<ListStruct>, start_year: i32, end_year: i32) -> Vec<ListStruct> {

    data_set.into_iter().filter(|x| x.year >= start_year && x.year <= end_year).collect()
}


// sort the data by list
fn sort_by_life_expectancy(mut data_set: Vec<ListStruct>, sort_order: bool) -> Vec<ListStruct> {

    if sort_order {
        data_set.sort_by(|a,b| a.life_expectancy.partial_cmp(&b.life_expectancy).unwrap());
    } else {
        data_set.sort_by(|a,b| b.life_expectancy.partial_cmp(&a.life_expectancy).unwrap());
    }

    data_set
}

fn sort_by_year(mut data_set: Vec<ListStruct>, sort_order: bool) -> Vec<ListStruct> {

    if sort_order {
        data_set.sort_by(|a,b| a.year.cmp(&b.year));
    } else {
        data_set.sort_by(|a,b| b.year.cmp(&a.year));
    }

    data_set
}

fn sort_by_delta(mut data_set: Vec<ListStruct>, sort_order: bool) -> Vec<ListStruct> {

    if sort_order  {
        data_set.sort_by(|a,b| a.delta.partial_cmp(&b.delta).unwrap());
    } else {
        data_set.sort_by(|a,b| b.delta.partial_cmp(&a.delta).unwrap());
    }

    data_set
}

fn sort_by_region(mut data_set: Vec<ListStruct>) -> Vec<ListStruct> {

    data_set.sort_by(|a,b| a.region.cmp(&b.region));

    data_set
}


// this function calculates the deltas
fn delta_query(data_set: Vec<ListStruct>, life_dict: HashMap<String, HashStruct>, year_delta: i32) -> Vec<ListStruct> {


    let mut return_data_set: Vec<ListStruct> = Vec::new();

    //iterate over dataset
    for r in data_set.iter() {


        // create key
        let year = (r.year + year_delta).to_string();
        let key = format!("{}:{}",r.region, year);
        // if it exists in the dictionary
        if life_dict.contains_key(&key.clone()) {

            // calculate the delta
            let delta_life_expectancy = life_dict.get(&key).unwrap().life_expectancy - r.life_expectancy;
            
            // create new row and append
            let mut row = r.clone();
            row.delta = delta_life_expectancy;
            return_data_set.push(row);
        }
    }

    return_data_set
 
 
}