use random_get::generate_numbers;
use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            eprintln!("Usage: random_get <count> <min> <max> <file>");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "random_get".to_owned());
    let count = parse_argument(&mut args, "count", &program)?.parse::<usize>()?;
    let min = parse_argument(&mut args, "min", &program)?.parse::<i64>()?;
    let max = parse_argument(&mut args, "max", &program)?.parse::<i64>()?;
    let file_name = parse_argument(&mut args, "file", &program)?;

    if args.next().is_some() {
        return Err("too many arguments".into());
    }

    let numbers = generate_numbers(count, min, max)?;
    let file = File::create(file_name)?;
    let mut output = BufWriter::new(file);

    for number in numbers {
        writeln!(output, "{number}")?;
    }

    output.flush()?;
    Ok(())
}

fn parse_argument<'a>(
    args: &mut impl Iterator<Item = String>,
    name: &str,
    program: &str,
) -> Result<String, io::Error> {
    args.next().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("missing {name} argument for {program}"),
        )
    })
}
