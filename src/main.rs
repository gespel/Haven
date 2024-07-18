use std::fs::{File};
use std::io;
use std::io::prelude::*;
use std::io::LineWriter;

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
    let mut projectname = String::new();
    let mut language = String::new();

    println!("Enter the project name: ");
    io::stdin().read_line(&mut projectname)?;
    let mut hw = HavenWriter::new(projectname.as_str());
    println!("Enter the project language: ");
    io::stdin().read_line(&mut language)?;

    match language.as_str().strip_suffix("\n").expect("Error while parsing input buffer!") {
        "rust" => {
            hw.create_dockerfile_rust().expect("Error while creating Dockerfile");
            println!("Dockerfile created!\n");
        }
        _ => {}
    }

    //hw.create_dockerfile_rust().expect("Error while writing Dockerfile!");

    Ok(())
}
