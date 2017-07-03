extern crate record;
extern crate kernel;

fn main() {
    let curr_time = kernel::current_time();
    let records = record::read_records_from("../data/data1617.csv");
    // let ker_sum = kernel::kernel_sum(&rec[10000], &curr_time);
    // println!("{}", ker_sum);
    for (i, rec) in records.iter()/*.skip(20000).take(20)*/.enumerate() {
        let ker_sum = kernel::kernel_sum(&rec, &curr_time);
        println!("{}> {}\n", i, ker_sum);
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