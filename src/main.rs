use polars::prelude::*;

fn get_pixo_filename() -> String {
    let args: Vec<String> = std::env::args().collect();
    let mut filename = String::from("test/res/pixometer_mixed_sources.csv");
    if args.len() == 2 {
        filename = String::from(args[1].as_str());
    }
    return filename;

}

fn main() {

    let file_to_convert = get_pixo_filename();
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
    schema.with_column(String::from("Ablesedatum"), DataType::Utf8); // this is a DateTime but with minutes resolution...
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

    println!("Original data:");
    println!("{:?}", data);

// ToDo turn into function 'split_time' taking dataframe to avoid the non telling name 'data2'
    let parse_time = |name: &str| {
        col(name).str().strptime(StrpTimeOptions {
            fmt: Some("%d.%m.%Y %H:%M".into()),
            ..Default::default()
        })
    };
    let data2 = data
        .lazy()
        .select(&[
            parse_time("Ablesedatum")
                .dt()
                .strftime("%d.%m.%Y")
                .alias("Ablesetag"),
            parse_time("Ablesedatum")
                .dt()
                .strftime("%H:%M")
                .alias("Ablesezeit"),
            col("Zählerstand"),
            col("Notes") + col("Notes_2"),
            col("Zählernummer")
        ])
        .collect()
        .unwrap();
    println!("After splitting date:");
    println!("{:?}", data2);

    // ToDo make function out of it to be a little bit more self-documenting
    println!("For each measurement:");
    let counter_num = data2.column("Zählernummer").unwrap();
    let counters = counter_num.unique().unwrap();
    
    for counter_id in 0..counters.len() {
        println!();

        let value = counters.get(counter_id);
        let id = match value {
            AnyValue::Utf8(str) => str,
            AnyValue::Utf8Owned(ref str) => str,
            value => panic!("Unimplemented / unexpected type {:?}", value),
        };
        let out_filename = format!("{id}.csv");
        println!("Writing following data to file {out_filename}:");
        let mut data = data2.filter(&counter_num.equal(id).unwrap()).unwrap().drop("Zählernummer").unwrap();
        println!(
            "{:?}",
            data
        );
        CsvWriter::new( std::fs::File::create(out_filename).unwrap())
        .has_header(false)
        .finish(&mut data)
        .unwrap();
    }
}
// ToDo convert to ecas format
