extern crate rustc_serialize;
extern crate quick_csv;
extern crate csv;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use quick_csv::Csv;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

#[derive(Debug, RustcDecodable, RustcEncodable, Clone)]
struct Record {
    values: Vec<String>,
}

#[derive(Debug, RustcDecodable, RustcEncodable, Clone)]
struct GeoRecord {
    record: Record,
    lat: f64,
    lng: f64,
}

fn read_records_from(path: &str, vec: &mut Vec<GeoRecord>) {
    let rdr = Csv::from_file(path).unwrap().has_header(true);
    for row in rdr.into_iter() {
        match row.unwrap().decode::<Record>() {
            Ok(cols) => {
                let geo_rec = GeoRecord {
                    record: cols,
                    lat: 0.0,
                    lng: 0.0
                };
                vec.push(geo_rec);
            }
            Err(error) => println!("{}", error),
        }
    }
}

fn write_records_to(path: &str, vec: &mut Vec<GeoRecord>) {
    let mut wtr = csv::Writer::from_file(path).unwrap();
    for record in vec.into_iter() {
        wtr.encode(record.clone());
    }
}

fn get_coordinates(vec: &mut Vec<GeoRecord>) {
    for record in vec.into_iter() {
        println!("http://maps.google.com/maps/api/geocode/json?address={},%20AT", record.record.values[10]);
    }
}

// fn coords() -> (f32, f32) {
//     unimplemented!()
// }

fn get_json_from(url: &str) -> Vec<u8> {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    let out = Vec::new();

    let uri = url.parse().unwrap();
    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().for_each(|chunk| {
            // out = chunk.first();
            io::stdout().write_all(&chunk).map(|_| ()).map_err(From::from)
        })
    });
    core.run(work).unwrap();
    out
}

fn main() {
    get_json_from("http://maps.google.com/maps/api/geocode/json?address=78%20MARIETTA%20ST,%20AT");

    let mut records = Vec::new();
    read_records_from("../../data/COBRA-YTD2017.csv", &mut records);
    get_coordinates(&mut records);
    write_records_to("../../data/out.csv", &mut records);
    // println!("{:?}", records);
}