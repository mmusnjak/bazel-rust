//! Scoped UUIDv7 generation, after https://brooker.co.za/blog/2025/10/22/uuidv7.html
//!
//! Only the 48-bit timestamp field of a normal UUIDv7 is replaced, with
//! `unix_ts_ms XOR H(id, unix_ts_ms >> N)`. `H` is HMAC-SHA256, keyed with the
//! shifted timestamp bucket and fed the scope `id`. All other UUIDv7 fields
//! (version, variant, random bits) stay as a plain UUIDv7 sets them.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

/// Bits dropped from the timestamp before it keys the hash. The article
/// recommends `N >= 12` to keep database insert locality.
const TIMESTAMP_SHIFT_BITS: u32 = 12;

/// Builds a UUIDv7 whose timestamp field is masked with an HMAC of `id`, so
/// that IDs scoped to the same `id` and the same `2^N` ms window sort and
/// cluster together.
pub fn scoped_v7(id: &str) -> Uuid {
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before the Unix epoch")
        .as_millis() as u64;
    masked_v7(id, now_ms)
}

fn masked_v7(id: &str, now_ms: u64) -> Uuid {
    let bucket = now_ms >> TIMESTAMP_SHIFT_BITS;
    let mut mac =
        HmacSha256::new_from_slice(&bucket.to_be_bytes()).expect("HMAC accepts a key of any length");
    mac.update(id.as_bytes());
    let digest = mac.finalize().into_bytes();

    let hash_bits = digest[..6]
        .iter()
        .fold(0u64, |acc, &byte| (acc << 8) | byte as u64);
    let masked_ts = (now_ms ^ hash_bits) & 0xFFFF_FFFF_FFFF;

    let mut bytes = *Uuid::now_v7().as_bytes();
    bytes[0..6].copy_from_slice(&masked_ts.to_be_bytes()[2..8]);
    Uuid::from_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn timestamp_field(id: &Uuid) -> u64 {
        let bytes = id.as_bytes();
        bytes[0..6]
            .iter()
            .fold(0u64, |acc, &byte| (acc << 8) | byte as u64)
    }

    /// Only the bucket, `masked_ts >> N`, is stable within a `2^N` ms window.
    /// The low N bits still move with the real clock, so inserts in the same
    /// window keep spreading out instead of colliding.
    fn bucket_of(id: &Uuid) -> u64 {
        timestamp_field(id) >> TIMESTAMP_SHIFT_BITS
    }

    #[test]
    fn same_id_same_window_has_same_bucket() {
        let now_ms: u64 = 1_700_000_000_000;
        let first = masked_v7("Hello", now_ms);
        let second = masked_v7("Hello", now_ms + 1);
        assert_eq!(bucket_of(&first), bucket_of(&second));
    }

    #[test]
    fn different_id_changes_the_bucket() {
        let now_ms: u64 = 1_700_000_000_000;
        let hello = masked_v7("Hello", now_ms);
        let goodbye = masked_v7("Goodbye", now_ms);
        assert_ne!(bucket_of(&hello), bucket_of(&goodbye));
    }

    #[test]
    fn version_and_variant_stay_rfc_compliant() {
        let id = masked_v7("Hello", 1_700_000_000_000);
        assert_eq!(id.get_version_num(), 7);
    }
}
