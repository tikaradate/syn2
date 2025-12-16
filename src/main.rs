use std::env;
use syn2::read_raw_wav;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: wavtool <path.wav>");
        return;
    }
    let file_path = &args[1];

    match read_raw_wav(file_path) {
        Ok(wav) => {
            println!("fmt: {:?}", wav.format());
            println!("data bytes: {}", wav.data_bytes().len())
        }
        Err(e) => eprintln!("error: {e}"),
    }
}
