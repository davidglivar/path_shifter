extern crate colored;
extern crate getopts;
extern crate rustc_serialize;

use colored::*;
use getopts::Options;
use rustc_serialize::json::{Json /*, ToJson, Object*/};
use std::env;
use std::error::Error;
use std::f64::{MAX};
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

    let mut input_str = String::new();
    match input_file.read_to_string(&mut input_str) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}", display, input_str),
    }

    let input_data = Json::from_str(&input_str).unwrap();
    println!("data: {}", input_data);

    if input_data.is_array() {
        shift_array(input_data);
    }
}

fn shift_array(json: Json) {
    let mut arr = json.as_array().unwrap();
    'a: for item in arr {
        let obj = match item.as_object() {
            Some(o) => { o },
            None => { continue 'a; },
        };

        let mut minx: f64 = MAX;
        let mut miny: f64 = MAX;

        'b: for (key, value) in obj.iter() {

            println!("key: {}, value: {}", key, value);
            match *value {
                Json::Array(ref a) => {
                    let resolved = a
                        .iter()
                        .map(|n| {
                            match *n {
                                Json::U64(i) => { i as f64 },
                                Json::I64(i) => { i as f64 },
                                Json::F64(i) => { i },
                                _ => { panic!("Unexpected type"); }
                            }
                        })
                        .collect::<Vec<f64>>();

                    for (idx, n) in resolved.iter().enumerate() {
                        if idx % 2 == 0 {
                            minx = n.min(minx);
                        } else {
                            miny = n.min(miny);
                        }
                    }
                },
                _ => { continue 'b; },
            }
            println!("minx: {}, miny: {}", minx, miny);
        }
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    println!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("o", "output", "set the output file path", "NAME");
    opts.optopt("x", "", "set the x point to translate to", "X");
    opts.optopt("y", "", "set the y point to translate to", "Y");

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
