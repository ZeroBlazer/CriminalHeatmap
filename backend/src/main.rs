extern crate record;
extern crate kernel;
extern crate clustering;

use std::collections::Bound::Included;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let week_range = 90;
    let threshold = 0.4;
    /*****************************LOAD RECORDS*******************************/
    let records = record::read_records_from("../data/data1617.csv"); // Load records
    let tree = clustering::make_record_tree(&records); // Store records in a BTree
    /****************************FILTER RECORDS******************************/
    let curr_time = kernel::current_time(); // Current Time
    let bound_time = kernel::weeks_ago(week_range); // Current Time - x Weeks

    let range_records = tree.range((Included(&bound_time), Included(&curr_time))); // Filter records in range
    /*********************KERNEL DENSITY ESTIMATION***************************/
    let mut output = File::create("../frontend/heatmap.js").expect("Error");
    output
        .write_all(b"function getPoints() {
    return [
        ")
        .expect("Error writing first part");

    let mut first = true;
    for (_, rec) in range_records {
        let ker_sum = kernel::kernel_sum(rec, &curr_time);
        if ker_sum > threshold {
            let (lat, lon) = rec.get_lat_lon();
            
            if !first {
                write!(output, ",\n\t\t").expect("Error in line break");
            } else {
                first = false;
            }

            write!(output,
                   "{{ location: new google.maps.LatLng({}, {}), weight: {} }}",
                   lat,
                   lon,
                   ker_sum)
                    .expect("Error writing record");
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
    /************************************************************************/
    println!("Hello world!");
}
