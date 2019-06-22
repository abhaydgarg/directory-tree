use std::{
  fs,
  process,
  path::Path
};
use clap::{
  Arg,
  App,
  crate_version,
  crate_authors
};
use spinners::{Spinner, Spinners};
use byte_unit::Byte;
use colored::*;

mod timer;
mod scanner;

fn main() {
  let matches = App::new("Directory tree")
    .version(crate_version!())
    .author(crate_authors!())
    .about("Creates a JSON representing a directory tree.")
    .arg(
      Arg::with_name("scan")
        .short("s")
        .long("scan")
        .number_of_values(2)
        .value_names(&["DIR", "FILE"])
        .required(true)
        .help("Provide directory to scan and file to write JSON")
        .takes_value(true),
    )
    .get_matches();

  let mut args = matches.values_of("scan").unwrap();
  let scan_dir = args.next().unwrap();
  let out_path = args.next().unwrap();

  // Validate <FILE> arg.
  let path = Path::new(out_path);
  if path.parent().unwrap().is_dir() == false {
    eprintln!(
      "{} {}",
      "Error".bright_red().bold(),
      "File's path does not exist"
    );
    process::exit(0);
  }
  if let Some(extension) = path.extension() {
    if extension.to_str().unwrap().trim().to_lowercase() != "json" {
      eprintln!(
        "{} {}",
        "Error".bright_red().bold(),
        "Invalid extension. Expected `.json`"
      );
      process::exit(0);
    }
  } else {
    eprintln!(
      "{} {}",
      "Error".bright_red().bold(),
      "Provide file name to write json. The file need not exist before hand"
    );
    process::exit(0);
  }

  // Begin scanning.
  let now = timer::start();

  let sp = Spinner::new(
    Spinners::Line,
    "Scanning, please wait... ".bright_cyan().bold().to_string(),
  );

  let result = match scanner::run(scan_dir) {
    Ok(r) => r,
    Err(e) => {
      eprintln!("{} {}", "Error".bright_red().bold(), e);
      process::exit(0);
    }
  };

  if let Err(e) = fs::write(out_path, result.json) {
    eprintln!("{} {}", "Error".bright_red().bold(), e);
    process::exit(0);
  }

  let byte = Byte::from_bytes(result.size as u128).get_appropriate_unit(true);
  let time = timer::end(now);
  sp.stop();

  println!("{}", "Done!".bright_green().bold());
  println!("{} {}", "Total time".bright_blue().bold(), time);
  println!("{} {}", "Scan time".bright_blue().bold(), result.time);
  println!("{} {}", "Size".bright_blue().bold(), byte.to_string());
}
