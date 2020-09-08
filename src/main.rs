use std::fs::File;
use std::io::{BufWriter, Write};

use clap::{App, Arg};

fn main() -> std::io::Result<()> {
    let args = App::new("json-sorter")
        .version("0.1")
        .about("sorts keys alphabetically in a JSON file")
        .arg(Arg::with_name("input_file")
            .help("The JSON file to sort keys in")
            .takes_value(true)
            .required(true))
        .get_matches();
    let input_file_name = args.value_of("input_file").unwrap();
    let json: serde_json::Value = serde_json::from_reader(File::open(input_file_name)
        .expect("file should open read only"))
        .expect("file should be proper JSON");
    BufWriter::new(
        File::create("sorted_".to_string() + input_file_name)
            .expect("Unable to create file [{}]"))
        .write_all(json.to_string().as_bytes())
        .expect("Unable to write data");
    Ok(())
}