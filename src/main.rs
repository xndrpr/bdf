use std::env;
use std::fmt::format;
use std::fs::File;
use std::io::{ self, BufRead, BufReader, BufWriter, Write };

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "No file path provided. Drag a file onto the .exe or pass a file path as an argument."
        );
        wait_for_user();
        return Ok(());
    }

    let input_path = &args[1];
    let required_height = input_path.split("h").last().unwrap().replace(".dat", "");
    let output_path = format!("output{}.dat", required_height);

    let in_file = File::open(input_path)?;
    let out_file = File::create(&output_path)?;

    let reader = BufReader::with_capacity(128 * 1024, in_file);
    let mut writer = BufWriter::with_capacity(128 * 1024, out_file);
    const TITLE: &str = "E\tN\tH\tS\n";
    writer.write_all(TITLE.as_bytes())?;

    for line_result in reader.lines().skip(1) {
        let line = line_result?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let height = parts[2];

        if height == required_height {
            writer.write_all(line.as_bytes())?;
            writer.write_all(b"\n")?;
        }
    }

    writer.flush()?;

    println!("Successfully processed '{}' and created '{}'.", input_path, output_path);

    wait_for_user();
    Ok(())
}

fn wait_for_user() {
    println!("\nPress Enter to exit...");
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
}
