extern crate chrono;
extern crate reqwest;
extern crate serde_json;
extern crate std;

use chrono::prelude::*;
use chrono::{DateTime, Local, TimeZone};
use serde_json::Value;
use std::collections::HashMap;

pub fn train_times() -> Result<Option<Vec<DateTime<Local>>>, Box<dyn std::error::Error>> {
    let station = "forhl";
    let dir_code = "1";
    let prediction_times = get_prediction_times(station, dir_code)?;
    let mut scheduled_times = get_scheduled_times(station, dir_code)?.unwrap_or(HashMap::new());
    if let Some(pred_times) = prediction_times {
        for key in pred_times.keys() {
            *scheduled_times.get_mut(key).unwrap() = pred_times[key]
        }
    }
    let now = Local::now();
    let mut all_times = scheduled_times
        .values()
        .filter_map(|date| if date > &now {Some(date.clone())}else{None})
        .collect::<Vec<DateTime<Local>>>();
    all_times.sort();
//    println!("{:?}", all_times);
    return Ok(Some(all_times));
}

fn get_prediction_times(
    station: &str,
    dir_code: &str,
) -> Result<Option<HashMap<String, DateTime<Local>>>, Box<dyn std::error::Error>> {
    let address = format!("https://api-v3.mbta.com/predictions?filter[stop]=place-{}&filter[direction_id]={}&include=stop&filter[route]=CR-Needham", station, dir_code);
    println!("{}", address);
    return get_rout_times(address);
}

fn get_scheduled_times(
    station: &str,
    dir_code: &str,
) -> Result<Option<HashMap<String, DateTime<Local>>>, Box<dyn std::error::Error>> {
    let now = chrono::Local::now();
    let address = format!("https://api-v3.mbta.com/schedules?include=route,trip,stop&filter[min_time]={}%3A{}&filter[stop]=place-{}&filter[route]=CR-Needham&filter[direction_id]={}",now.hour(), now.minute(), station, dir_code);
    return get_rout_times(address);
}

fn get_rout_times(
    address: String,
) -> Result<Option<HashMap<String, DateTime<Local>>>, Box<dyn std::error::Error>> {
    let routes_json: Value = reqwest::blocking::get(&address)?.json()?;
    let data_option = routes_json.get("data");
    if let Some(data) = data_option {
        if let Some(data_array) = data.as_array() {
            let mut commuter_rail_dep_time: HashMap<String, chrono::DateTime<Local>> =
                HashMap::new();
            for train in data_array {
                let departure_time_option = train["attributes"]["departure_time"].as_str();
                let trip_id_option = train["relationships"]["trip"]["data"]["id"].as_str();
                if let Some(trip_id) = trip_id_option {
                    if let Some(departure_time) = departure_time_option {
                        let departure_time_datetime =
                            Local.datetime_from_str(departure_time, "%+")?;
                        commuter_rail_dep_time.insert(trip_id.to_string(), departure_time_datetime);
                    }
                }
            }
 //           println!("{:?}", commuter_rail_dep_time);
            return Ok(Some(commuter_rail_dep_time));
        } else {
            return Ok(None);
        }
    } else {
        return Ok(None);
    };
}
