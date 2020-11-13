use clap::{Arg, App};

fn main() {
    //TODO push to github

    //parse cmd args
    let matches = App::new("Csv Generator")
        .version("0.1.0")
        .about("Generates csvs with random data")
        .arg(Arg::with_name("schema")
                 .short("s")
                 .long("schema")
                 .takes_value(true)
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
    println!("The output will be written to: {}", outfile);        
    //parse schema toml
    //generate csv from schema
    //BONUS concurent line generation, feed to writer
    
    //println!("Hello, world!");
}


/*
    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Your favorite number must be {}.", n + 5),
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }
}
*/