use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::process::{Command, Child};


#[derive(Debug)]
pub struct Launcher {
    pub description: String,
    pub command: String,
    pub parameters: String,
    pub working_dir: PathBuf,
}

impl Launcher{
    pub fn launch(&self) -> io::Result<Child> {
        Command::new(&self.command)
            .args(self.parameters.split_ascii_whitespace())
            .current_dir(&self.working_dir)
            .spawn()
    }
}

pub struct LaunchData {
    programs: Vec<Launcher>,
    mapping: HashMap<String, usize>,
}

impl LaunchData {
    pub fn new() -> LaunchData {
        LaunchData {
            programs: Vec::new(),
            mapping: HashMap::new(),
        }
    }
    pub fn add(&mut self, launcher: Launcher, listen: Vec<String>) {
        let index = self.programs.len();
        self.programs.push(launcher);
        for s in listen {
            self.mapping.insert(s.to_string(), index);
        }
    }
    pub fn trigger(&self, s: &str) {
        match self.mapping.get(s) {
            Some(index) => {
                let launcher = &self.programs[*index];
                println!("Launching {:?}", launcher);
                if let Err(err) = launcher.launch(){
                    println!("Error launching {}: {:?}", &launcher.command, err);
                }
            }
            None => {
                println!("Request \"{}\" unrecognized.", s);
            }
        }
    }
}
