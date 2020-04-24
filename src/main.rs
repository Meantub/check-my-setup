// #![allow(unused_imports)] // WARN LATER

extern crate path_absolutize;
use path_absolutize::*;

extern crate clap;
use clap::{Arg, App};

use std::fs::File;
use std::io::Read;

use std::path::Path;
use std::result::Result;

// Use these to create symlinks in Windows
#[cfg(target_os = "windows")]
use std::os::windows::fs::{symlink_dir, symlink_file};
// Use these symlinks when in Unix
#[cfg(target_os = "unix")]
use std::os::unix::fs::{symlink};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Setup {
    name: String,
    r#type: Type,
    source_location: Option<String>,
    target_location: Option<String>
}

#[derive(Deserialize, Debug)]
enum Type {
    File,
    Directory
}

impl ToString for Type {
    fn to_string(&self) -> String {
        return match &self {
            Type::File => String::from("File"),
            Type::Directory => String::from("Directory")
        }
    }
}

fn main() {
    let matches = App::new("CheckMySetup")
        .version("1.0")
        .author("Kenneth T. <kentubman5@gmail.com>")
        .about("Allows you to put all your configs in one directory")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets custom config file")
            .takes_value(true)
            .default_value("setup.json"))
        .get_matches();

    let config = matches.value_of("config").unwrap();

    let file_contents = read_file_contents(config).unwrap();

    let setups: Vec<Setup> = convert_to_setup(file_contents).unwrap();

    for setup in setups {
        let source_location_absolutize = Path::new(&setup.source_location.unwrap()).absolutize().unwrap();
        let target_location_absolutize = Path::new(&setup.target_location.unwrap()).absolutize().unwrap();

        println!("Linking: {:?}  to {:?}", source_location_absolutize, target_location_absolutize);
        create_symlink(source_location_absolutize, target_location_absolutize, setup.r#type);
    }
}

fn read_file_contents<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return Ok(contents);
}

fn convert_to_setup(json_string: String) -> Result<std::vec::Vec<Setup>, serde_json::Error> {
    let setups: Vec<Setup> = serde_json::from_str(&json_string).unwrap();
    return Ok(setups);
}

#[cfg(target_os = "windows")]
fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q, r#type: Type) -> () {
    match r#type {
        Type::File => {
            if dst.as_ref().exists() {
                return;
            }

            symlink_file(src, &dst).unwrap()
        },
        Type::Directory => {
            // if the destination exists then we can cancel symlink
            if dst.as_ref().exists() {
                return;
            }

            symlink_dir(src, &dst).unwrap();
        }
    };
}

#[cfg(target_os = "unix")]
fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q, _type: Type) -> () {
    symlink(src, dst).unwrap();
}
