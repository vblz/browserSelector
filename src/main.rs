#![windows_subsystem = "windows"]

// extern crate winreg;
// extern crate serde_derive;
//
// use std::{env, fs};
// use std::io;
// use std::path::PathBuf;
//
// use winreg::RegKey;
// use winreg::enums::*;
//
// use serde_derive::Deserialize;
// use std::collections::HashMap;
//
// use std::process::Command;
//
// use std::error::Error;

use std::env;

mod open;
mod registry;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        std::process::exit(1);
    }

    let arg = args[1].as_str();

    let result = match arg {
        "register" => register(env::current_exe(), file_in_exe_dir("ico.ico")),
        "unregister" => unregister(),
        _ => open(arg),
    };

    if let Result::Err(err) = result {
        println!("error: {}", err)
    }
}