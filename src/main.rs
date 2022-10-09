#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
use std::fs;
use std::io;
use shasher;
use std::io::Write;
use std::fs::File;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::prelude::*;



#[derive(Debug)]
struct Node {
    path: PathBuf,
    files: Vec<PathBuf>,
    hashes: Vec<String>,
    folders: Vec<Box<Node>>,
}

impl Node {
    pub fn new(path: PathBuf) -> Self {
        Node {
            path: path,
            files: Vec::new(),
            hashes: Vec::new(),
            folders: Vec::new(),
        }
    }
    
    pub fn burrow(&mut self) {
        let mut contents: Vec<PathBuf> = ls_dir(&self.path);

        for item in contents {
            if item.is_file() {
                let hashed_file: String = item.as_path().display().to_string();
                
                println!("{}", &hashed_file);

                self.hashes.push(format!("{:0x}", shasher::get_hash(&102, &hashed_file[..])));
            } else if item.is_dir() {
                let mut new_folder = Node::new(item);
                new_folder.burrow();
                self.folders.push(Box::new(new_folder));
            }
        }
    }

    pub fn write_hashes(&self, path: &str) {
        let mut file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(path)
                        .unwrap();

        for hash in &self.hashes {
            if let Err(e) = writeln!(file, "{}", hash) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
        
        if self.folders.len() > 0 {
            for folder in &self.folders {
                folder.write_hashes(path);
            }
        }
        
    }

    pub fn show(&self) {
        println!("PATH: {}", self.path.as_path().display());
        println!("HASHES: {:?}", self.hashes);
        for folder in &self.folders {
            folder.show();
        }
    }
}

fn ls_dir(path: &PathBuf) -> Vec<PathBuf> {
    let mut contents: Vec<PathBuf> = Vec::new();
    let current_dir: fs::ReadDir = fs::read_dir(path).unwrap();

    for file in current_dir {
        contents.push(file.unwrap().path());
    }
    contents
}

fn main() {
    let root_path: PathBuf = PathBuf::from("/home/nimrafets/projects/rust/binaries/shadir_walker");
    let contents: Vec<PathBuf> = ls_dir(&root_path);
    let mut node: Node = Node::new(root_path);
   
    node.burrow();
    
    let hash_path: &str = "/home/nimrafets/projects/rust/binaries/shadir_walker/hashes.txt";
    let mut to_file: File = match File::create(hash_path) {
        Ok(_file) => _file,
        Err(_e) => panic!("Error creating file {}", hash_path),
    };
    
    node.write_hashes(hash_path);
}
