use std::io;

use polars::prelude::*;

fn main() {

    println!("Hello, which file do you want to convert?");
    //let mut file_to_convert = String::new();
    //io::stdin()
    //    .read_line(&mut file_to_convert)
    //    .expect("Path to file couldn't be read");
    // ToDo reenable the above and let the program prompt for the file to read
    let file_to_convert = "test/res/pixometer_mixed_sources.csv".to_string();
    println!(
        "Converting {} into the ecas format and writing output to current folder",
        file_to_convert
    );

    let mut schema = Schema::new();
    schema.with_column(String::from("Standort"), DataType::Utf8);
    schema.with_column(String::from("Sparte"), DataType::Utf8);
    schema.with_column(String::from("Zählernummer"), DataType::Utf8);
    schema.with_column(String::from("Zählerstand"), DataType::Float64);
    schema.with_column(String::from("Zählerstand_NT"), DataType::Float64);
    schema.with_column(String::from("Ablesedatum"), DataType::Utf8); // this is a DateTIme but with Minutes resolution... 
    schema.with_column(String::from("Notes"), DataType::Utf8);
    schema.with_column(String::from("Notes_2"), DataType::Utf8);
    
    let data: DataFrame = CsvReader::from_path(file_to_convert)
        .expect("bar")
        .has_header(false)
        .with_schema(&schema)
        .with_parse_dates(true)
        .with_ignore_parser_errors(true)
        .with_skip_rows(1)
        .finish()
        .expect("bar");
    // ToDo convert Ablesedatum into datetime
    // ToDo convert to ecas format
    println!("{:?}", data);
}
