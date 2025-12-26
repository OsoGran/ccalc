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

    let data: Vec<u16> = vec![0xDEAD, 0xBEEF, 0xCAFE];
    let data_slice: &[u16] = &data;

    let output: u16 = checksum_1760(data_slice);

    println!("Checksum: {:?}", output);
}

fn checksum_1760(data: &[u16]) -> Result<&u16, &str> {
    // Handle empty slice
    data.first().ok_or("Slice was empty")

    let mut checksum = 0;
    for (index, word) in data.iter().enumerate(){
        checksum = checksum ^ (word.rotate_right(index));
    }
    Ok(checksum);
}
