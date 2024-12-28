mod config;

use std::env;
use std::fs::File;
use std::io::{ self, BufRead, BufReader, BufWriter, Write };

use config::Config;

fn wait_for_user() {
    println!("\nPress Enter to exit...");
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
}

fn init() -> io::Result<Config> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "No file path provided. Drag a file onto the .exe or pass a file path as an argument."
        );
        wait_for_user();

        std::process::exit(1);
    }

    let input_path = &args[1];
    let input_file = File::open(input_path)?;
    let param = input_path.split("h").last().unwrap().replace(".dat", "");
    let output_path = format!("output{}.dat", param);
    let output_file = File::create(&output_path)?;
    let config = Config::new(&input_path, &output_path, input_file, output_file, param);

    return Ok(config);
}

fn main() -> io::Result<()> {
    let config = init()?;

    let reader = BufReader::with_capacity(128 * 1024, config.input_file);
    let mut writer = BufWriter::with_capacity(128 * 1024, config.output_file);
    writer.write_all(config.title.as_bytes())?;

    let is_range = config.param.split("R").count() > 1;
    if is_range {
        let range = config.param
            .split("R")
            .into_iter()
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();

        for line_result in reader.lines().skip(1) {
            let line = line_result?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            let value = parts[config.param_index].parse::<f64>().unwrap();

            if value <= range[0] && value >= range[1] {
                writer.write_all(line.as_bytes())?;
                writer.write_all(b"\n")?;
            }
        }
    } else {
        let param_value = config.param.parse::<f64>().unwrap();

        for line_result in reader.lines().skip(1) {
            let line = line_result?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            let value = parts[config.param_index].parse::<f64>().unwrap();

            if value == param_value {
                writer.write_all(line.as_bytes())?;
                writer.write_all(b"\n")?;
            }
        }
    }

    writer.flush()?;

    println!(
        "Successfully processed '{}' and created '{}'.",
        config.input_path,
        config.output_path
    );

    wait_for_user();
    Ok(())
}
