use std::io::{BufReader, BufRead};
use std::fs::File;
use serde::{Serialize, Deserialize};
use std::fs;

const VERSION: u32 = 0;

// DO NOT change datatype from f64, cpp expects double datatype
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Document {
    version: u32,
    dimesions: u32,
    vertexes: Vec<Point>
}


// TODO: add check to make sure dimesions matches the length of coordinates
impl Document {
    fn new(version: u32, dimensions: u32) -> Self {
        Document { version: version, dimesions: dimensions, vertexes: vec![]}
    }

    fn add_vertexes(&mut self, point: Point) {
        self.vertexes.push(point);
    }
}

pub(crate) fn wavefront_obj_parser(filepath: &str) -> Result<Document, Box<dyn std::error::Error>> {
    // TODO: check if file is valid wavefront obj
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut document = Document::new(VERSION, 3);
    
    for line in reader.lines() {
        let line = line?;
        // wavefront obj vertext coordinates start with "v "
        if line.starts_with("v ") {
            let tokenized = line.split(" ");
            let tokens: Vec<&str> = tokenized.collect();
            
            let x: f64 = tokens[1].parse().unwrap();
            let y: f64 = tokens[2].parse().unwrap();
            let z: f64 =  tokens[3].parse().unwrap();

            document.add_vertexes(Point{x:x, y:y, z:z});
        }
    }

    Ok(document)
}

pub(crate) fn save_to_disk(filepath: &str, document: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(filepath, document).expect("can't save to disk");
    Ok(())
}