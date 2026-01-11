use convert_case::{Case, Casing};
use regex::Regex;
use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

fn main() {
    let current_dir = env::current_dir().unwrap();
    let src_dir = current_dir.join("src");

    println!("cargo:rerun-if-changed={}", src_dir.display());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out = out_dir.join("problems.rs");
    let out = File::create(out).expect("Failed to create problems.rs");
    let mut writer = BufWriter::new(out);

    let re = Regex::new(r"\d\.([\w-]+)\.rs").unwrap();

    let problems = src_dir
        .read_dir()
        .expect("Failed to read src dir")
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .filter_map(|e| {
            let path = e.path();
            let file_name = e.file_name();
            let file_name = file_name.to_str().unwrap();

            re.captures(file_name)
                .and_then(|c| c.get(1))
                .map(|c| (path, c.as_str().to_string()))
        });

    for (path, name) in problems {
        writeln!(
            writer,
            "#[path = {:?}] mod {};",
            path,
            name.to_case(Case::Snake)
        )
        .unwrap();
    }

    writer.flush().unwrap();
}
