use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The algorithm to look for
    algo: String,
    /// The path to look for
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("algorithm: {:?}, path: {:?}", args.algo, args.path)
}

fn checksum_1760(data: &[u16]) -> u16 {
    //
}
