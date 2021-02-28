use std::path::{PathBuf};
use structopt::StructOpt;
use serde_json;

use google_authenticator_extractor::{
    Account,
    extract_from_uri,
    extract_text_from_qrcode
};

#[derive(Debug, StructOpt)]
#[structopt(name = "google-authenticator-extractor", about = "Extract information from the qrcode exported from google authenticator.")]
struct Opt {
    #[structopt(short, parse(from_os_str))]
    image: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt: Opt = Opt::from_args();
    let text = extract_text_from_qrcode(opt.image)?;
    let accounts: Vec<Account> = extract_from_uri(text.as_str())?;

    let x = serde_json::to_string(&accounts)?;

    println!("{}", x);

    Ok(())
}