use std::collections::HashMap;
use std::io;
use std::process::{Child, Command};
use log::{info,warn,error};

#[derive(Debug)]
pub struct Launcher {
    pub description: String,
    pub command: String,
    pub parameters: String,
    pub working_dir: String,
}
impl Launcher {
    pub fn launch(&self) -> io::Result<Child> {
        let mut cmd = Command::new(&self.command);
        cmd.args(self.parameters.split_ascii_whitespace());
        if !self.working_dir.is_empty() {
            cmd.current_dir(&self.working_dir);
        }
        let result = cmd.spawn();
        result
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
                info!("Launching {:?}", launcher);
                if let Err(err) = launcher.launch() {
                    error!("Error launching {}: {:?}", &launcher.command, err);
                }
            }
            None => {
                warn!("Request \"{}\" unrecognized.", s);
            }
        }
    }
}
