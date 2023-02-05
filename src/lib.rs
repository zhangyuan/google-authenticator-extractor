use base64::{Engine as _, engine::general_purpose};

use protobuf::Message;
use serde::Serialize;
use std::error::Error;
use std::fmt::Formatter;
use std::path::Path;

mod protos;
#[derive(Debug, Serialize)]
pub struct Account {
    name: String,
    secret: String,
    issuer: String,
}

impl Account {
    fn new(name: String, secret: String, issuer: String) -> Account {
        Account {
            name,
            secret,
            issuer,
        }
    }
}

pub fn extract_text_from_qrcode<P>(image_path: P) -> Result<String, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let image = image::open(image_path)?;
    let image = image.to_luma8();
    let mut img = rqrr::PreparedImage::prepare(image);
    let grids = img.detect_grids();
    let (_meta, text) = grids[0].decode()?;
    Ok(text)
}

pub fn extract_from_uri(text: &str) -> Result<Vec<Account>, Box<dyn std::error::Error>> {
    let encoded_data = extract_data_from_uri(text)?;
    let data = general_purpose::STANDARD.decode(encoded_data)?;
    let data_in_bytes = data.as_slice();

    let migration_payload = protos::google_auth::MigrationPayload::parse_from_bytes(data_in_bytes)?;

    let otp_parameters = migration_payload.otp_parameters.into_vec();

    let alphabet = base32::Alphabet::RFC4648 { padding: false };

    let payloads: Vec<Account> = otp_parameters
        .into_iter()
        .map(|p| {
            Account::new(
                p.name,
                base32::encode(alphabet, p.secret.as_slice()),
                p.issuer,
            )
        })
        .collect();

    Ok(payloads)
}

fn extract_data_from_uri(raw: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut split = raw.split("data=");
    split.next();
    if let Some(encoded_data) = split.next() {
        let s = urlencoding::decode(encoded_data)?;
        Ok(s.to_string())
    } else {
        Err(Box::new(ExtractorError::InvalidDataError))
    }
}

#[derive(Debug)]
pub enum ExtractorError {
    InvalidDataError,
}

impl Error for ExtractorError {}

impl ::std::fmt::Display for ExtractorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ExtractorError::InvalidDataError => "InvalidDataError(data from qrcode is invalid)",
        };
        write!(f, "{}", msg)
    }
}
