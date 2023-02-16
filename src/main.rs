extern crate csv;
extern crate levenshtein;

use csv::StringRecord;
use levenshtein::levenshtein;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io;
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
        ammo_vec.push(record.clone());
        if let Some(name) = record.get(0) {
            name_vec.push(name.to_string());
        }
    }
    Ok(())
}

fn find_ammo(query_ammo: String, name_vec: &mut Vec<String>) -> usize {
    let mut similarity_vec: Vec<usize> = Vec::new();

    for name in name_vec {
        similarity_vec.push(levenshtein(&query_ammo, &name));
    }
    let minimum_similarity = similarity_vec.iter().min().unwrap();
    similarity_vec
        .iter()
        .position(|r| r == minimum_similarity)
        .unwrap()
}

fn main() {
    // check file existing
    let file_path = match get_first_arg() {
        Ok(file_path) => file_path,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    let mut name_vec: Vec<String> = Vec::new(); // vector for ammo names
    let mut ammo_vec: Vec<StringRecord> = Vec::new(); // vector for ammo data

    // read ammo data csv
    match csv_parse(fopen(&file_path).unwrap(), &mut name_vec, &mut ammo_vec) {
        Ok(_) => println!("Successfully parsed CSV"),
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }

    let column_list = [
        "Name",
        "ShortName",
        "InternalName",
        "Type",
        "Damage",
        "PenetrationPower",
        "MaxArmorPenClass",
        "ArmorDamage",
        "Speed",
        "IsTracer",
        "MisfireChance",
        "PenetrationPowerDeviation",
        "PenetrationChance",
        "RicochetChance",
        "SpeedRetardation",
        "Description",
    ];

    loop {
        println!("Input what ammo you want to know");
        // input ammo name to find ammo
        let mut query_ammo: String = String::new();
        let result: io::Result<usize> = io::stdin().read_line(&mut query_ammo);
        match result {
            Ok(_) => {
                //println!("{}", query_ammo);
            }
            Err(err) => {
                println!("{}", err);
            }
        }

        let most_similar_ammo_idx: usize = find_ammo(query_ammo, &mut name_vec);

        for (info, column_name) in ammo_vec
            .get(most_similar_ammo_idx)
            .unwrap()
            .iter()
            .zip(column_list.iter())
        {
            println!("{}: {:?}", column_name, info);
        }
        println!("\n");
    }
}
