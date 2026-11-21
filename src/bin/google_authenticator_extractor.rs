use std::error::Error;
use std::path::{Path, PathBuf};

use structopt::StructOpt;

use google_authenticator_extractor::{
    Account, extract_batch_from_uri, extract_text_from_qrcode, merge_batches, missing_batch_indexes,
};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "google-authenticator-extractor",
    about = "Extract information from the qrcode exported from google authenticator."
)]
struct Opt {
    /// One or more QR code images (or directories) exported from Google
    /// Authenticator. Pass every image of a multi-QR export to get all
    /// accounts.
    #[structopt(short, long, parse(from_os_str), required = true, min_values = 1)]
    image: Vec<PathBuf>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let paths = expand_paths(&opt.image)?;

    let mut batches = Vec::new();
    let mut failed = 0;

    for path in &paths {
        let result = extract_text_from_qrcode(path).and_then(|text| extract_batch_from_uri(&text));

        match result {
            Ok(batch) => batches.push(batch),
            Err(error) => {
                failed += 1;
                eprintln!("{}: {error}", path.display());
            }
        }
    }

    if batches.is_empty() {
        return Err(format!("could not extract any accounts ({failed} image(s) failed)").into());
    }

    for (batch_id, index) in missing_batch_indexes(&batches) {
        if batch_id == 0 {
            eprintln!("Warning: QR code {} is missing from the export.", index + 1);
        } else {
            eprintln!(
                "Warning: QR code {} of export batch {batch_id} is missing.",
                index + 1
            );
        }
    }

    if failed > 0 {
        eprintln!("Warning: {failed} image(s) could not be processed.");
    }

    let accounts: Vec<Account> = merge_batches(&batches);
    println!("{}", serde_json::to_string(&accounts)?);

    Ok(())
}

/// Expand directories into the image files they contain.
fn expand_paths(inputs: &[PathBuf]) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut paths = Vec::new();

    for input in inputs {
        if input.is_dir() {
            let mut entries: Vec<PathBuf> = std::fs::read_dir(input)?
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| path.is_file() && is_image(path))
                .collect();
            entries.sort();
            paths.extend(entries);
        } else {
            paths.push(input.clone());
        }
    }

    if paths.is_empty() {
        return Err("no input images found".into());
    }

    Ok(paths)
}

fn is_image(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some("png" | "jpg" | "jpeg" | "webp" | "bmp" | "gif" | "tif" | "tiff")
    )
}
