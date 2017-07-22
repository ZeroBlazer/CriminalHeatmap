extern crate record;
extern crate kernel;
/************************************************************************/
extern crate rustc_serialize;
extern crate csv;

#[derive(RustcEncodable)]
pub struct NaiveRecord(String, String, f64);
/************************************************************************/
use std::io::prelude::*;
use std::fs::File;
/************************************************************************/
fn main() {
    let curr_time = kernel::current_time();
    let records = record::read_records_from("../data/data1617.csv");
    // let filtrd: Vec<(String, String, f64)> = Vec::new();

    // let mut wtr = csv::Writer::from_file("../frontend/kde.csv").unwrap();
    // for (i, rec) in records.iter().enumerate() {
    //     let ker_sum = kernel::kernel_sum(&rec, &curr_time);
    //     if ker_sum > 0.6 {
    //         println!("{}> {}\n", i, ker_sum);
    //         let (lat, lon) = rec.get_lat_lon();
    //         let record = NaiveRecord(lat, lon, ker_sum * 10.0); //"lat","lon","weight"
    //         wtr.encode(record);
    //     }
    // }

    let mut output = File::create("../frontend/heatmap.js").expect("Error");
    output
        .write_all(b"function getPoints() {
    return [
        ")
        .expect("Error writing first part");

    let mut first = true;
    for rec in &records {
        let ker_sum = kernel::kernel_sum(&rec, &curr_time);
        if ker_sum > 0.4 {
            // THRESHOLD
            if !first {
                write!(output, ",\n\t\t").expect("Error in line break");
            } else {
                first = false;
            }

            let (lat, lon) = rec.get_lat_lon();
            write!(output,
                   "{{ location: new google.maps.LatLng({}, {}), weight: {} }}",
                   lat,
                   lon,
                   ker_sum).expect("Error writing record");
        }
    }

    output
        .write_all(b"    ];
}

heatmap = new google.maps.visualization.HeatmapLayer({
    data: getPoints(),
    map: map
});")
        .expect("Error writing third part");

    println!("Hello world!");
}
