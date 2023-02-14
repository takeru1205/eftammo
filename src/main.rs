extern crate csv;

use csv::StringRecord;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn fopen(path: &OsString) -> Result<File, Box<dyn Error>> {
    let file = File::open(path)?;
    Ok(file)
}

fn csv_parse(
    file: File,
    name_vec: &mut Vec<String>,
    ammo_vec: &mut Vec<StringRecord>,
) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
        ammo_vec.push(record.clone());
        if let Some(name) = record.get(0) {
            name_vec.push(name.to_string());
        }
    }
    Ok(())
}

fn main() {
    let file_path = match get_first_arg() {
        Ok(file_path) => file_path,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    let mut name_vec: Vec<String> = Vec::new();
    let mut ammo_vec: Vec<StringRecord> = Vec::new();

    match csv_parse(fopen(&file_path).unwrap(), &mut name_vec, &mut ammo_vec) {
        Ok(_) => println!("Successfully parsed CSV"),
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
    println!("{:?}", name_vec);
    println!("{:?}", ammo_vec);
    println!("length of name_vec is {}", name_vec.len());
    println!("length of ammo_vec is {}", ammo_vec.len());
}
