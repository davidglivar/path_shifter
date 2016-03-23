extern crate colored;
extern crate getopts;
extern crate rustc_serialize;

use colored::*;
use getopts::Options;
use rustc_serialize::json::Json;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn do_work(input: &str, output: Option<String>) {
    let out = match output {
        Some(x) => { x }
        None => { panic!("No output!") }
    };

    println!("{}: {}", "input".bold(), input);
    println!("{}: {}", "output".bold(), out);

    let input_path = Path::new(input);
    let display = input_path.display();
    println!("{}", display);

    let mut input_file = match File::open(&input_path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    println!("{:?}", input_file);

    let mut input_str = String::new();
    match input_file.read_to_string(&mut input_str) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}", display, input_str),
    }

    let input_data = Json::from_str(&input_str).unwrap();
    println!("data: {}", input_data);
    println!("data is object? {}", input_data.is_object());
    println!("data is array? {}", input_data.is_array());

    shift_array(&input_data);
    // input_data would be inaccessible here if i did not pass it as a reference
    println!("data is array? {}", input_data.is_array());
}

fn shift_array(json: &Json) {
    println!("ok {}", json.is_array());
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    println!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "output", "set the output file path", "NAME");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let output = matches.opt_str("o");
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    do_work(&input, output);
}
