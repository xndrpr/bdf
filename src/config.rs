use std::fs::File;

pub struct Config {
    pub input_path: String,
    pub output_path: String,
    pub input_file: File,
    pub output_file: File,
    pub title: String,
    pub param: String,
    pub param_index: usize,
}

impl Config {
    pub fn new(
        input_path: &str,
        output_path: &str,
        input_file: File,
        output_file: File,
        param: String
    ) -> Config {
        const TITLE: &str = "E\tN\tH\tS\n";

        Config {
            input_path: input_path.to_string(),
            output_path: output_path.to_string(),
            input_file,
            output_file,
            title: TITLE.to_string(),
            param,
            param_index: 2,
        }
    }
}
