use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use clap::{App, Arg};

fn main() -> std::io::Result<()> {
    let args = App::new("json-sorter")
        .version("0.1")
        .about("sorts keys alphabetically in a JSON file")
        .arg(Arg::with_name("inputFile")
            .help("The JSON file to sort keys in")
            .takes_value(true)
            .required(true))
        .get_matches();
    let input_file_name = args.value_of("inputFile").unwrap();
    let input_file = fs::File::open(input_file_name)
        .expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(input_file)
        .expect("file should be proper JSON");
    let output_file_name = "sorted_".to_string() + input_file_name;
    let output_file = File::create(output_file_name).expect("Unable to create file [{}]");
    let mut output_file_writer = BufWriter::new(output_file);
    output_file_writer.write_all(json.to_string().as_bytes()).expect("Unable to write data");
    Ok(())
}