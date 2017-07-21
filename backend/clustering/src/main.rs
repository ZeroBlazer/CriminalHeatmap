extern crate record;
extern crate kernel;
extern crate clustering;
extern crate rustc_serialize;
extern crate csv;
extern crate cogset;

use std::collections::Bound::Included;
use cogset::{Dbscan, BruteScan, Euclid};

fn main() {
    let curr_time = kernel::current_time(); // Current Time
    let bound_time = kernel::weeks_ago(7); // Current Time - x Weeks

    let records = record::read_records_from("../../data/data1617.csv"); // Load records
    let tree = clustering::make_record_tree(&records); // Store records in a BTree

    let range_records = tree.range((Included(&bound_time), Included(&curr_time))); // Filter records in range
    /************************CLUSTERING***********************/
    let query_records: Vec<record::Record> = range_records.map(|(_, &r)| r.clone()).collect(); // Vec containing cloned query records
    let scanner = BruteScan::new(&query_records);
    let mut dbscan = Dbscan::new(scanner, 0.01, 5);

    let clusters = dbscan.by_ref().collect::<Vec<_>>();
    /**********************************************************/
    for (i, cluster) in clusters.iter().enumerate() {
        println!("\nCLUSTER {}>", i);
        for elem_idx in cluster {
            println!("- {:?}", query_records[*elem_idx].get_lat_lon());
        }
    }
    // println!("{:#?}", clusters);
}
