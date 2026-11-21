use base64::{
    Engine as _,
    engine::general_purpose::{STANDARD, STANDARD_NO_PAD, URL_SAFE, URL_SAFE_NO_PAD},
};
use image::GrayImage;
use image::imageops::{self, FilterType};

use protobuf::Message;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::error::Error;
use std::fmt::Formatter;
use std::path::Path;

mod protos;
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
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

/// One QR code of a (possibly multi-QR) Google Authenticator export.
///
/// Exports with more than roughly ten accounts are split across several QR
/// codes. Each one carries the same `batch_id` and its own
/// `batch_index`/`batch_size`.
#[derive(Clone, Debug, Serialize)]
pub struct MigrationBatch {
    pub accounts: Vec<Account>,
    pub version: i32,
    pub batch_size: i32,
    pub batch_index: i32,
    pub batch_id: i32,
}

/// Don't upscale images beyond this many pixels; it would only waste memory.
const MAX_UPSCALED_PIXELS: u64 = 64_000_000;

/// Above this size, an image without any detected QR code is not retried at a
/// higher resolution. Upscaling cannot reveal a code that detection already
/// missed, and it is expensive.
const MAX_NO_GRID_UPSCALE_PIXELS: u64 = 1_000_000;

pub fn extract_text_from_qrcode<P>(image_path: P) -> Result<String, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let image = image::open(image_path)?.to_luma8();
    extract_text_from_image(&image)
}

fn extract_text_from_image(image: &GrayImage) -> Result<String, Box<dyn std::error::Error>> {
    // qr codes with many modules are sensitive to how well the module grid
    // lines up with the pixels of the image. A screenshot taken at a
    // non-integer number of pixels per module can make rqrr sample a few rows
    // or columns off, which shows up as `DataEcc` even though the image looks
    // perfectly readable. Upscaling the image first usually fixes this, so try
    // the original image and a couple of upscaled variants before giving up.
    let mut last_error: Option<Box<dyn std::error::Error>> = None;

    for (scale, filter) in candidate_scales(image) {
        let candidate = if scale == 1 {
            image.clone()
        } else {
            imageops::resize(image, image.width() * scale, image.height() * scale, filter)
        };

        match decode_all_grids(&candidate) {
            Ok(text) => return Ok(text),
            Err(error) => {
                // Keep a real decoding error (e.g. DataEcc) in preference to
                // "no qr code found", since it says more about what went wrong.
                let is_not_found = error
                    .downcast_ref::<ExtractorError>()
                    .is_some_and(|e| matches!(e, ExtractorError::QrCodeNotFound));

                if scale == 1
                    && is_not_found
                    && image.width() as u64 * image.height() as u64 > MAX_NO_GRID_UPSCALE_PIXELS
                {
                    return Err(error);
                }

                if !is_not_found || last_error.is_none() {
                    last_error = Some(error);
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| Box::new(ExtractorError::QrCodeNotFound)))
}

fn candidate_scales(image: &GrayImage) -> Vec<(u32, FilterType)> {
    let mut attempts = vec![(1, FilterType::Nearest)];

    for scale in [2, 3, 4] {
        let pixels = (image.width() as u64 * scale as u64) * (image.height() as u64 * scale as u64);
        if pixels > MAX_UPSCALED_PIXELS {
            continue;
        }

        attempts.push((scale, FilterType::Lanczos3));
        attempts.push((scale, FilterType::CatmullRom));
        attempts.push((scale, FilterType::Nearest));
    }

    attempts
}

fn decode_all_grids(image: &GrayImage) -> Result<String, Box<dyn std::error::Error>> {
    let mut prepared = rqrr::PreparedImage::prepare(image.clone());
    let grids = prepared.detect_grids();

    if grids.is_empty() {
        return Err(Box::new(ExtractorError::QrCodeNotFound));
    }

    let mut last_error: Option<Box<dyn std::error::Error>> = None;

    for grid in &grids {
        match grid.decode() {
            Ok((_meta, text)) => return Ok(text),
            Err(error) => last_error = Some(Box::new(error)),
        }

        // Also accept mirrored codes, e.g. when the image was flipped.
        if let Ok((_meta, text)) = rqrr::Grid::new(MirroredGrid(&grid.grid)).decode() {
            return Ok(text);
        }
    }

    Err(last_error.unwrap())
}

struct MirroredGrid<'a, G: rqrr::BitGrid + ?Sized>(&'a G);

impl<G: rqrr::BitGrid + ?Sized> rqrr::BitGrid for MirroredGrid<'_, G> {
    fn size(&self) -> usize {
        self.0.size()
    }

    fn bit(&self, y: usize, x: usize) -> bool {
        self.0.bit(x, y)
    }
}

pub fn extract_from_uri(text: &str) -> Result<Vec<Account>, Box<dyn std::error::Error>> {
    Ok(extract_batch_from_uri(text)?.accounts)
}

pub fn extract_batch_from_uri(text: &str) -> Result<MigrationBatch, Box<dyn std::error::Error>> {
    let encoded_data = extract_data_from_uri(text)?;
    let data = decode_base64(&encoded_data)?;

    let migration_payload = protos::google_auth::MigrationPayload::parse_from_bytes(&data)?;

    let otp_parameters = migration_payload.otp_parameters.to_vec();

    let alphabet = base32::Alphabet::Rfc4648 { padding: false };

    let accounts: Vec<Account> = otp_parameters
        .into_iter()
        .map(|p| {
            Account::new(
                p.name,
                base32::encode(alphabet, p.secret.as_slice()),
                p.issuer,
            )
        })
        .collect();

    Ok(MigrationBatch {
        accounts,
        version: migration_payload.version,
        batch_size: migration_payload.batch_size,
        batch_index: migration_payload.batch_index,
        batch_id: migration_payload.batch_id,
    })
}

/// Merge the batches of one or more exports into a single, de-duplicated list.
///
/// Exports are kept in the order they were supplied, while the QR codes inside
/// each export are ordered by `batch_index`.
pub fn merge_batches(batches: &[MigrationBatch]) -> Vec<Account> {
    let mut export_order: Vec<i32> = Vec::new();
    for batch in batches {
        if !export_order.contains(&batch.batch_id) {
            export_order.push(batch.batch_id);
        }
    }

    let mut seen = HashSet::new();
    let mut merged = Vec::new();

    for batch_id in export_order {
        let mut export: Vec<&MigrationBatch> = batches
            .iter()
            .filter(|batch| batch.batch_id == batch_id)
            .collect();
        export.sort_by_key(|batch| batch.batch_index);

        for batch in export {
            for account in &batch.accounts {
                if seen.insert(account.clone()) {
                    merged.push(account.clone());
                }
            }
        }
    }

    merged
}

/// Return `(batch_id, batch_index)` pairs that are missing from `batches`.
///
/// A batch is considered missing when its export advertises a `batch_size`
/// larger than the number of distinct batches that were supplied. Single-QR
/// exports (`batch_size <= 0`) are ignored.
pub fn missing_batch_indexes(batches: &[MigrationBatch]) -> Vec<(i32, i32)> {
    let mut by_batch_id: BTreeMap<i32, (i32, BTreeSet<i32>)> = BTreeMap::new();

    for batch in batches {
        if batch.batch_size <= 0 {
            continue;
        }

        let entry = by_batch_id.entry(batch.batch_id).or_default();
        entry.0 = entry.0.max(batch.batch_size);
        entry.1.insert(batch.batch_index);
    }

    let mut missing = Vec::new();
    for (batch_id, (batch_size, seen)) in by_batch_id {
        for index in 0..batch_size {
            if !seen.contains(&index) {
                missing.push((batch_id, index));
            }
        }
    }

    missing
}

fn decode_base64(input: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let input = input.trim();

    // Google Authenticator emits standard base64, while other exports use the
    // URL-safe alphabet and sometimes omit padding. Accept both.
    for engine in [STANDARD, URL_SAFE, STANDARD_NO_PAD, URL_SAFE_NO_PAD] {
        if let Ok(decoded) = engine.decode(input) {
            return Ok(decoded);
        }
    }

    let unpadded = input.trim_end_matches('=');
    for engine in [STANDARD_NO_PAD, URL_SAFE_NO_PAD] {
        if let Ok(decoded) = engine.decode(unpadded) {
            return Ok(decoded);
        }
    }

    Err(Box::new(ExtractorError::InvalidDataError))
}

fn extract_data_from_uri(raw: &str) -> Result<String, Box<dyn std::error::Error>> {
    let query = raw.split_once('?').map_or(raw, |(_, query)| query);

    let value = query
        .split('&')
        .filter_map(|pair| pair.split_once('='))
        .find_map(|(key, value)| (key == "data").then_some(value))
        .ok_or(ExtractorError::InvalidDataError)?;

    Ok(urlencoding::decode(value)?.into_owned())
}

#[derive(Debug)]
pub enum ExtractorError {
    InvalidDataError,
    QrCodeNotFound,
}

impl Error for ExtractorError {}

impl ::std::fmt::Display for ExtractorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ExtractorError::InvalidDataError => "InvalidDataError(data from qrcode is invalid)",
            ExtractorError::QrCodeNotFound => "QrCodeNotFound(no qr code found in the image)",
        };
        write!(f, "{msg}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn account(name: &str) -> Account {
        Account::new(
            name.to_string(),
            "JBSWY3DPEHPK3PXP".to_string(),
            "Issuer".to_string(),
        )
    }

    fn batch(id: i32, index: i32, size: i32, accounts: Vec<Account>) -> MigrationBatch {
        MigrationBatch {
            accounts,
            version: 1,
            batch_size: size,
            batch_index: index,
            batch_id: id,
        }
    }

    #[test]
    fn merges_batches_in_batch_order() {
        let batches = vec![
            batch(7, 1, 2, vec![account("c")]),
            batch(7, 0, 2, vec![account("a"), account("b")]),
        ];

        let names: Vec<_> = merge_batches(&batches)
            .into_iter()
            .map(|account| account.name)
            .collect();

        assert_eq!(names, ["a", "b", "c"]);
    }

    #[test]
    fn keeps_supplied_export_order() {
        let batches = vec![
            batch(20, 0, 1, vec![account("a")]),
            batch(10, 0, 1, vec![account("b")]),
        ];

        let names: Vec<_> = merge_batches(&batches)
            .into_iter()
            .map(|account| account.name)
            .collect();

        assert_eq!(names, ["a", "b"]);
    }

    #[test]
    fn deduplicates_accounts_across_batches() {
        let batches = vec![
            batch(0, 0, 1, vec![account("a")]),
            batch(0, 0, 1, vec![account("a")]),
        ];

        assert_eq!(merge_batches(&batches).len(), 1);
    }

    #[test]
    fn reports_missing_batch_indexes() {
        let batches = vec![
            batch(42, 0, 3, vec![account("a")]),
            batch(42, 2, 3, vec![account("b")]),
        ];

        assert_eq!(missing_batch_indexes(&batches), vec![(42, 1)]);
    }

    #[test]
    fn ignores_single_qr_exports() {
        assert!(missing_batch_indexes(&[batch(0, 0, 0, vec![account("a")])]).is_empty());
        assert!(missing_batch_indexes(&[batch(0, 0, 1, vec![account("a")])]).is_empty());
    }

    #[test]
    fn parses_url_safe_base64_without_padding() {
        let uri = "otpauth-migration://offline?data=CjEKCkhlbGxvId6tvu8SGEV4YW1wbGU6YWxpY2VAZ29vZ2xlLmNvbRoHRXhhbXBsZTAC";

        let batch = extract_batch_from_uri(uri).unwrap();

        assert_eq!(batch.accounts.len(), 1);
        assert_eq!(batch.accounts[0].name, "Example:alice@google.com");
        assert_eq!(batch.accounts[0].issuer, "Example");
    }
}
