extern crate quick_csv;
extern crate rustc_serialize;
extern crate chrono;

use quick_csv::Csv;
use std::io::{self, Write, Read};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Record {
    values: Vec<String>,
}

impl Record {
    pub fn get_date_time(&self) -> NaiveDateTime {
        println!("D: {}, T: {}, WD: {}",
                 self.values[3],
                 self.values[4],
                 self.values[16]);
        let date: Vec<&str> = self.values[3].split("/").collect();
        let mut time: Vec<&str> = self.values[4].split(":").collect();
        
        if time.len() == 1 {
            time.pop();
            time.push("00");
            time.push("00");
            time.push("00");
        }

        let dt = NaiveDate::from_ymd(i32::from_str_radix(date[2], 10).unwrap(),
                                     u32::from_str_radix(date[0], 10).unwrap(),
                                     u32::from_str_radix(date[1], 10).unwrap())
                .and_hms(u32::from_str_radix(time[0], 10).unwrap(),
                         u32::from_str_radix(time[1], 10).unwrap(),
                         u32::from_str_radix(time[2], 10).unwrap());
        
        dt
    }

    pub fn get_lat_lon(&self) -> (String, String) {
        (self.values[22].clone(), self.values[21].clone())
    }
}

pub fn read_records_from(path: &str) -> Vec<Record> {
    let mut vec = Vec::new();
    let rdr = Csv::from_file(path).unwrap().has_header(true);
    for row in rdr.into_iter() {
        match row.unwrap().decode::<Record>() {
            Ok(vals) => {
                vec.push(vals);
            }
            Err(error) => println!("{}", error),
        }
    }

    vec
}