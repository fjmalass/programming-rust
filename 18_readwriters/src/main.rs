// Getting traits for read/write
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

use colored::*;
use std::error::Error;

use std::process::{Command, Stdio};

// Binary files
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
//
// Serialization
use serde::{Deserialize, Serialize};
use serde_json;




#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    name: String,
    items: Vec<String>,
    health: u64,
}

fn grep<R>(target: &str, reader: R) -> io::Result<()>
where
    R: BufRead,
{
    for line in reader.lines() {
        let line = line?;
        if line.contains(target) {
            println!("Found: {} in '{}'", &target, line);
        }
    }
    Ok(())
}

fn grep_main() -> Result<(), Box<dyn Error>> {
    // get command line and remove the first argument as it is the name of the program
    let mut args = std::env::args().skip(1);
    let target = match args.next() {
        Some(s) => s,
        None => Err("usage: grep PATTERN FILE...")?,
    };

    let files: Vec<PathBuf> = args.map(PathBuf::from).collect();
    if files.is_empty() {
        let stdin = io::stdin();
        grep(&target, stdin.lock())?;
    } else {
        for file in files {
            let f = File::open(file)?;
            grep(&target, io::BufReader::new(f))?;
        }
    }

    Ok(())
}

fn main_fn() {
    if let Err(e) = grep_main() {
        println!("{}", e);
        std::process::exit(1);
    }
}

fn main() {
    {
        println!("{:=^49}", " STDIN ".green());
        // main_fn()
        // let stdin = io::stdin();
        // grep("rust", stdin.lock()).unwrap();
    }

    {
        println!("{:=^49}", " COLLECT LINES ".green());
        let mut lines = vec![];
        let reader = io::BufReader::new(File::open("Cargo.toml").unwrap());
        for line_result in reader.lines() {
            let line = line_result.unwrap();
            lines.push(line);
        }

        // better way
        let reader = io::BufReader::new(File::open("Cargo.toml").unwrap());
        let lines: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
        println!("{:?}", lines);
    }
    {
        println!("{:=^49}", " WRITER ".green());
    }
    {
        println!("{:=^49}", " COMMAND ".green());
        let mut child = Command::new("grep")
            .arg("-e")
            .arg("a. *.u")
            .stdin(Stdio::piped())
            .spawn();

        let mut to_child = child.as_mut().expect("Not Implented").stdin.take().unwrap();
        let my_words = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i"];
        for word in my_words {
            writeln!(to_child, "{}", word);
        }
        drop(to_child);
        child.expect("Not implemented").wait();
    }
    {
        println!("{:=^49}", " BINARY - COMPRESSION ".green());
        let mut reader = io::BufReader::new(File::open("Cargo.toml").unwrap());
        let n = reader.read_u32::<LittleEndian>().unwrap();
        println!("n = {}", n);
    }
    {
        println!("{:=^49}", " JSON ".green());
        let player = Player {
            name: "John".to_string(),
            items: vec!["axe".to_string(), "sword".to_string()],
            health: 100,
        };
        println!("Player = {:?}", player);
        serde_json::to_writer(std::io::stdout(), &player).unwrap();
    }

}
