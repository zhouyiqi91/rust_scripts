use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    };

    run(config);
}

struct Config {
    query: String,
    filename: String,
}

//https://stackoverflow.com/questions/65674207/why-does-a-function-accept-vecstring-when-its-parameter-is-of-type-string?noredirect=1&lq=1
fn parse_config(args: &Vec<String>) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    OK(())
}