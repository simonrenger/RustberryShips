extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str;
use std::io::Read;

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

    let manifest_folder = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_resource_folder = Path::new(&manifest_folder).join("resources");
    println!("Manifest_resource: {}", manifest_resource_folder.to_str().unwrap());
    let configuration = env::var("PROFILE").unwrap();
    let build_folder = Path::new(&manifest_folder).join(String::from("target")).join(&configuration);
    println!("build output folder: {}", build_folder.to_str().unwrap());
    let resource_output_folder = Path::new(&manifest_folder).join(String::from("target")).join(&configuration).join("resources");
    println!("resource_output: {}", resource_output_folder.to_str().unwrap());


    // Copies the resources/ folder to the output folder, not needed actually :(
    //Command::new("cmd").args(&["/C", "mkdir", resource_output_folder.to_str().unwrap()]).spawn()?;
    //Command::new("cmd").args(&["/C", "xcopy", manifest_resource_folder.to_str().unwrap(), resource_output_folder.to_str().unwrap(), "/D", "/E", "/C", "/R", "/Y", "/K"]).spawn()?;
}
