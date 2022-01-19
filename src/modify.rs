/*
TABLE
|----------|
|2|hi|94725| <- ROW
|19|6|FIELD|
|----------|
*/

use std::fs;
use chrono;

pub struct Table<'a> { // Table
    pub path: &'a str, // name of db, absolute or relative path
}

pub struct Row { // Row
    pub pos: i32, // position (line) in Table
}

pub struct Field {
    pub pos: i32, // position in Row
}

impl Table<'_> {
    pub fn create(path: &str) -> i8 {
        println!("creating in {}", path);
        let mut name = path.clone();
        if path.contains("/") || path.contains(r#"\"#) {
            let (_, substr) = path.rsplit_once('/').unwrap();
            name = substr;
        }
        let info = format!("jadb database\ntablename: {}\ncreated on: {}\npath: {}", name, chrono::offset::Local::now(), path);
        fs::create_dir(path).expect("Couldn't create db directory.");
        fs::write(format!("{}/{}", path, "info.jadb"), info).expect("Couldn't create info file.");
        0 // if ok return 0
    }
    pub fn write(content: &str, table: Table, row: i32) -> i8 {
        println!("Writing {} to table path {} in Row {}", content, table.path, row);
        0 // if ok return 0
    }
}

#[derive(Debug)]
pub enum LenType {
    Fields,
    Characters,
}

impl Row {
    pub fn length(utype: LenType) -> i32 {
        dbg!(&utype);
        -1 // if not ok return -1
    }
    pub fn hash() -> &'static str {
        "error" // if not ok return error
    }
}

impl Field {
    pub fn length() -> i32 {
        -1 // if not ok return -1
    }
    pub fn hash() -> &'static str {
        "error" // if not ok return error
    }
}