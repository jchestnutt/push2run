use std::path::{Path, PathBuf};
use std::error::Error;
use std::fs;
extern crate json;
use crate::launcher;

// Fields used in the Push2Run export file.
static DESCRIPTION: &str = "Descrption";
static COMMAND: &str = "Open";
static LISTEN_FOR: &str = "ListenFor";
static PARAMETERS: &str = "Parameters";
static START_IN: &str = "StartIn";

pub fn load_data<P: AsRef<Path>>(path: P) -> Result<launcher::LaunchData, Box<dyn Error>> {
    let json_data: json::JsonValue =
        json::parse(&fs::read_to_string(path).expect("Could not read file"))
            .expect("Could not parse JSON");

    let mut launchdata = launcher::LaunchData::new();
    for entry in json_data.members() {
        let listen_for: String = entry[LISTEN_FOR].as_str().unwrap().to_string();
        let triggers: Vec<String> = listen_for.split("\r\n").map(|x| x.to_string()).collect();
        launchdata.add(
            launcher::Launcher {
                description: entry[DESCRIPTION].as_str().unwrap().to_string(),
                command: entry[COMMAND].as_str().unwrap().to_string(),
                parameters: entry[PARAMETERS].as_str().unwrap().to_string(),
                working_dir: PathBuf::from(entry[START_IN].as_str().unwrap()),
            },
            triggers,
        );
        println!("Added {:?}", entry[DESCRIPTION].as_str().unwrap());
    }
    Ok(launchdata)
}
