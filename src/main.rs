use std::fs::{File};
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
            dockerfile: LineWriter::new(File::create(project_name.to_string()).expect("Cannot create Dockerfile")),
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

fn main() {
    let mut hw = HavenWriter::new("Haven");
    hw.create_dockerfile_rust().expect("Error while writing Dockerfile!");
    println!("Dockerfile created!\n");
}
