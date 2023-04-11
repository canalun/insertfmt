use insertfmt_core::format_insert_queries;
use std::{
    env,
    error::Error,
    fs::OpenOptions,
    io::{Read, Write},
    process,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut failed_paths: Vec<&str> = Vec::new();
    for path in args.iter().skip(1) {
        if let Err(err) = fmt_inserts(path) {
            println!("failed to format the file: {path}\nerror: {err}\n");
            failed_paths.push(path);
            continue;
        }
        println!("succeeded to format the file: {path}\n");
    }
    if failed_paths.len() > 0 {
        println!(
            "result\n👻 failed to format the following files: {}",
            failed_paths.join(", ")
        );
        process::exit(1);
    } else {
        println!("result\n🎉 succeeded to format all the files!!");
        process::exit(0);
    }
}

fn fmt_inserts(path: &str) -> Result<(), Box<dyn Error>> {
    let mut f = OpenOptions::new().read(true).open(path)?;
    let mut sql = String::new();
    f.read_to_string(&mut sql)?;

    let formatted_queries = format_insert_queries(&sql)?;

    let mut f = OpenOptions::new().truncate(true).write(true).open(path)?;
    f.write(formatted_queries.as_bytes())?;

    return Ok(());
}
