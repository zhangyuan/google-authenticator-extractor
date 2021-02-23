use base32;
use base64;
use protobuf::Message;
use rqrr;
use urlencoding;
use std::path::{PathBuf, Path};
use structopt::StructOpt;
use std::error::Error;
use std::fmt::Formatter;
use serde::Serialize;
use serde_json;

mod protos;

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

#[derive(Debug, Serialize)]
struct Account {
    name: String,
    secret: String
}

impl Account {
    fn new(name: String, secret: String) -> Account {
        Account {
            name,
            secret
        }
    }
}

fn extract_text_from_qrcode<P>(image_path: P) -> Result<String, Box<dyn std::error::Error>>
where P: AsRef<Path>
{
    let image = image::open(image_path)?;
    let image = image.to_luma8();
    let mut img = rqrr::PreparedImage::prepare(image);
    let grids = img.detect_grids();
    let (_meta, text) = grids[0].decode()?;
    Ok(text)
}

fn extract_from_uri(text: &str) -> Result<Vec<Account>, Box<dyn std::error::Error>>
{
    let encoded_data = extract_data_from_uri(text)?;
    let data = base64::decode(encoded_data)?;
    let data_in_bytes = data.as_slice();

    let migration_payload = protos::google_auth::MigrationPayload::parse_from_bytes(data_in_bytes)?;

    let otp_parameters = migration_payload.otp_parameters.into_vec();

    let alphabet = base32::Alphabet::RFC4648 { padding: false };

    let payloads: Vec<Account> = otp_parameters.into_iter().map(
        |p| Account::new(p.name, base32::encode(alphabet, p.secret.as_slice()))
    ).collect();

    Ok(payloads)
}


fn extract_data_from_uri(raw: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut split = raw.split("data=");
    split.next();
    if let Some(encoded_data) = split.next() {
        let x = urlencoding::decode(encoded_data)?;
        Ok(x)
    } else {
        Err(Box::new(ExtractorError::InvalidDataError))
    }
}

#[derive(Debug)]
pub enum ExtractorError {
    InvalidDataError
}

impl Error for ExtractorError {}

impl ::std::fmt::Display for ExtractorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ExtractorError::InvalidDataError => "InvalidDataError(data from qrcode is invalid)"
        };
        write!(f, "{}", msg)
    }
}
