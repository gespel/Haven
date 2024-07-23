#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs::{File};
use std::{env, io};
use std::io::prelude::*;
use std::io::LineWriter;
use std::path::{Path, PathBuf};
use crate::Language::{PYTHON, RUST, UNKOWN};

#[derive(Debug)]
enum Language {
    RUST,
    PYTHON,
    CLANG,
    CPPLANG,
    UNKOWN
}

struct HavenScanner {

}
impl HavenScanner {
    fn new() -> HavenScanner {
        HavenScanner {

        }
    }

    fn scan(&self) -> Language {
        //let path = env::current_dir().expect("Cannot determine current directory");
        if self.file_exists("Cargo.toml") {
            return RUST
        }
        else if self.file_exists("requirements.txt") {
            return PYTHON
        }
        return UNKOWN
    }

    fn file_exists(&self, file_name: &str) -> bool {
        return if Path::new(file_name).exists() {
            true
        } else {
            false
        }
    }
}


struct HavenWriter {
    project_name: String,
    dockerfile: LineWriter<File>
}

impl HavenWriter {
    fn new(project_name: &str) -> HavenWriter {
        HavenWriter {
            project_name: project_name.to_string().clone(),
            dockerfile: LineWriter::new(File::create("Dockerfile").expect("Cannot create Dockerfile")),
        }
    }

    fn create_dockerfile_rust(&mut self) -> std::io::Result<()> {
        let project_name = self.project_name.as_str();
        self.dockerfile.write_all(b"FROM rust:latest\n\n")?;
        self.dockerfile.write_all(b"COPY . .\n\n")?;
        self.dockerfile.write_all(b"RUN cargo build --release\n\n")?;
        self.dockerfile.write_all(format!("CMD [\"./target/release/{project_name}\"]\n\n").as_bytes())?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let hs = HavenScanner::new();
    let mut projectname = String::new();
    let lang = hs.scan();
    println!("Scanned current directory and found {:?} as primary language.", lang);

    println!("Enter the project name: ");
    io::stdin().read_line(&mut projectname)?;
    let mut hw = HavenWriter::new(projectname.as_str().strip_suffix("\n").unwrap());

    match lang {
        Language::RUST => {
            hw.create_dockerfile_rust().expect("");
        }
        Language::PYTHON => {}
        Language::CLANG => {}
        Language::CPPLANG => {}
        Language::UNKOWN => {}
    }



    /*match language.as_str().strip_suffix("\n").expect("Error while parsing input buffer!") {
        "rust" => {
            hw.create_dockerfile_rust().expect("Error while creating Dockerfile");
            println!("Dockerfile created!\n");
        }
        _ => {}
    }*/

    //hw.create_dockerfile_rust().expect("Error while writing Dockerfile!");



    Ok(())
}
