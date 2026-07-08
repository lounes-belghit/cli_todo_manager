use crate::todoitem::Item;
//use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
pub fn read_json(path_: &str) -> Vec<Item> {
    let file = if std::path::Path::new(path_).exists() {
        File::open(path_).unwrap()
    } else {
        File::create(path_).unwrap()
    };
    let reader = BufReader::new(file);
    let list: Vec<Item> = serde_json::from_reader(reader).unwrap_or(vec![]); //unwrap_(); //vec![];
    return list;
}
pub fn write_json(path_: &str, list: &Vec<Item>) -> bool {
    /*let list: Vec<Item> = if std::path::Path::new(path_).exists() {
        read_json(path_)
    } else {
        vec![]
    };*/
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) //delete the data inside the file
        .open(path_)
        .unwrap();
    let writer = BufWriter::new(file);
    match serde_json::to_writer_pretty(writer, &list) {
        Ok(_) => return true,
        _ => return false,
    }
}
