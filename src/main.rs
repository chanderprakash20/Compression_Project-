use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, Read};
use std::thread::sleep;
use std::time::Duration;
use flate2::write::GzEncoder;
use flate2::Compression;
use indicatif::{ProgressBar, ProgressStyle};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: <program> <source> <target>");
        return Ok(());
    }

    let source_path = &args[1];
    let target_path = &args[2];

    let mut bufread = BufReader::new(File::open(source_path)?);
    let bufwrite = BufWriter::new(File::create(target_path)?);

    let total_bytes = bufread.get_ref().metadata()?.len();
    let bar = ProgressBar::new(total_bytes);
    bar.set_draw_target(indicatif::ProgressDrawTarget::stdout());
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .progress_chars("#>-")
    );

    let mut encoder = GzEncoder::new(bufwrite, Compression::default());
    let mut buffer = [0; 256]; // Reduced buffer size for more updates

    loop {
        let bytes_read = bufread.read(&mut buffer)?;
        if bytes_read == 0 { break; }
        encoder.write_all(&buffer[..bytes_read])?;
        bar.inc(bytes_read as u64);
        sleep(Duration::from_millis(50)); // Delay for demonstration
    }

    bar.finish_with_message("Done");
    encoder.finish()?;
    Ok(())
}
