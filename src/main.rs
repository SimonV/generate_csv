use clap::{Arg, App};

fn main() {

    //parse cmd args
    let matches = App::new("Csv Generator")
        .version("0.1.0")
        .about("Generates csvs with random data")
        .arg(Arg::with_name("schema")
                 .short("s")
                 .long("schema")
                 .takes_value(true)
                 .required(true)
                 .help("A schema file"))
        .arg(Arg::with_name("outfile")
                 .short("o")
                 .long("outfile")
                 .takes_value(true)
                 .help("Output file"))                 
        .arg(Arg::with_name("rows")
                 .short("r")
                 .long("rows")
                 .takes_value(true)
                 .help("Number of data rows to generate"))
        .get_matches();
        
    let outfile = matches.value_of("outfile").unwrap_or("output.csv");
    let schema_file = matches.value_of("schema").unwrap();
    let rows = matches.value_of("rows").unwrap_or("10").parse::<i32>().expect("Failed to parse 'rows' argument");
    println!("The output will be written to: {}, using schema file: {}, generating {} rows", outfile, schema_file, rows);
    
    //parse schema toml
    //generate csv from schema
    //BONUS concurent line generation, feed to writer
    
    //println!("Hello, world!");
}