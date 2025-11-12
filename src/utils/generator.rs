use std::io::Error;
use crate::logger::Logger;
use crate::pkmn::Move;

#[cfg(target_os = "windows")]
pub const GENERATOR_BIN: &[u8] = include_bytes!("../../bin/generator.exe");

#[derive(Default)]
pub struct Generator {
    pub version: Option<String>,
    pub species: Option<usize>,
    pub nickname: Option<String>,
    pub trainer_name: Option<String>,
    pub level: Option<usize>,
    pub ball: Option<usize>,
    pub gender: Option<usize>,
    pub nature: Option<String>,
    pub shiny: Option<String>,
    pub moves: Option<Vec<Move>>,
    pub form: Option<usize>,
    pub ivs: Option<Vec<usize>>,
    pub evs: Option<Vec<usize>>,
}

impl Generator {
    pub fn new() -> Generator {
        Self::default()
    }

    /// Create a string of args
    fn args(&self) -> String {
        let mut args: Vec<String> = Vec::new();

        if let Some(v) = &self.version {
            args.push(format!("--version {}", v));
        }
        if let Some(s) = self.species {
            args.push(format!("--species {}", s));
        }
        if let Some(n) = &self.nickname {
            args.push(format!("--nickname {}", n));
        }
        if let Some(t) = &self.trainer_name {
            args.push(format!("--trainer-name {}", t));
        }
        if let Some(l) = self.level {
            args.push(format!("--level {}", l));
        }
        if let Some(b) = self.ball {
            args.push(format!("--ball {}", b));
        }
        if let Some(g) = self.gender {
            args.push(format!("--gender {}", g));
        }
        if let Some(n) = &self.nature {
            args.push(format!("--nature {}", n));
        }
        if let Some(s) = &self.shiny {
            args.push(format!("--shiny {}", s));
        }
        if let Some(f) = self.form {
            args.push(format!("--form {}", f));
        }
        if let Some(ivs) = &self.ivs {
            args.push(format!("--ivs {}", ivs.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")));
        }
        if let Some(evs) = &self.evs {
            args.push(format!("--evs {}", evs.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")));
        }
        if let Some(moves) = &self.moves {
            for (i, m) in moves.iter().enumerate() {
                args.push(format!("--move{} {}", i + 1, m.name));
            }
        }

        args.join(" ")
    }

    pub fn run(self) {
        match run_generator(self) {
            Ok(_) => {
                Logger::info("Generator complete.");
            }
            Err(e) => {
                Logger::error(e.to_string());
            }
        };
    }
}

#[cfg(target_os = "windows")]
fn run_generator(_settings: Generator) -> std::io::Result<()> {
    use std::{env, fs, process::Command};

    let path = env::temp_dir().join("generator.exe");
    fs::write(&path, GENERATOR_BIN)?;

    Command::new(&path)
        .arg("--example")
        .spawn()?;

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn run_generator(_settings: Generator) -> std::io::Result<()> {
    let msg = format!("Generator not available on this platform: {}", std::env::consts::OS.to_string());

    Logger::info(msg.clone());
    Ok(())
}
