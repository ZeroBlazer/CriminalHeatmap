extern crate record;
extern crate kernel;

/************************************************************************/
extern crate rustc_serialize;
extern crate csv;

#[derive(RustcEncodable)]
pub struct NaiveRecord(String, String, f64);
/************************************************************************/
fn main() {
    let curr_time = kernel::current_time();
    let records = record::read_records_from("../data/data1617.csv");
    let filtrd: Vec<(String, String, f64)> = Vec::new();
    // let ker_sum = kernel::kernel_sum(&rec[10000], &curr_time);
    // println!("{}", ker_sum);
    let mut wtr = csv::Writer::from_file("../frontend/kde.csv").unwrap();
    for (i, rec) in records.iter()/*.skip(20000).take(20)*/.enumerate() {
        let ker_sum = kernel::kernel_sum(&rec, &curr_time);
        if ker_sum > 0.6 {
            println!("{}> {}\n", i, ker_sum);
            let (lat, lon) = rec.get_lat_lon();
            let record = NaiveRecord(lat, lon, ker_sum * 10.0);    //"lat","lon","weight"
            wtr.encode(record);
        }
    }

    println!("Hello world!");
}

// extern crate iron;
// extern crate geojson;

// use geojson::GeoJson;
// use iron::prelude::*;
// use iron::status;

// fn main() {
//     Iron::new(|_: &mut Request| {
//         Ok(Response::with((status::Ok, "Hello World!")))
//     }).http("localhost:3000").unwrap();
// }