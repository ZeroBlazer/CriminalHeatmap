extern crate record;
extern crate kernel;
extern crate clustering;
extern crate rustc_serialize;
extern crate csv;

use std::collections::Bound::Included;

fn main() {
    let curr_time = kernel::current_time(); // Current Time
    let bound_time = kernel::weeks_ago(15); // Current Time - x Weeks

    let records = record::read_records_from("../../data/data1617.csv"); // Load records
    let tree = clustering::make_record_tree(&records);                  // Store records in a BTree

    let range_records = tree.range((Included(&bound_time), Included(&curr_time))); // Filter records in range
    for record in range_records {
        println!("{:?}", record);
    }
}
