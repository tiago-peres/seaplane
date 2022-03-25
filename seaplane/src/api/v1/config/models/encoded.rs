use std::fmt;

use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};

/// Holds a [URL-safe base64 encoded](https://datatracker.ietf.org/doc/html/rfc4648#section-5) string
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub(crate) struct Base64Encoded(String);

impl fmt::Display for Base64Encoded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Base64Encoded {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Base64Encoded {
    /// Constructs from an unencoded byte array, encoding with URL-safe base64 in the process
    pub(crate) fn from_unencoded(unencoded: impl AsRef<[u8]>) -> Self {
        Base64Encoded(encode_config(unencoded, URL_SAFE_NO_PAD))
    }

    /// Constructs a `Base64Encoded`, assuming the input is already encoded.
    // This is a reasonable thing to provide, as the majority of the time this function will be used
    // with the keys that are returned by the API, which are already encoded
    pub(crate) fn from_encoded(encoded: impl Into<String>) -> Self {
        Base64Encoded(encoded.into())
    }

    /// Returns the result of decoding the inner string.
    /// # Panics
    /// Will panic if the inner string is not correctly encoded.
    pub(crate) fn decoded(&self) -> Vec<u8> {
        decode_config(&self.0, URL_SAFE_NO_PAD)
            .expect("failed to decode, should be safe by construction")
    }

    /// Returns the inner string
    pub(crate) fn encoded(&self) -> &str {
        &self.0
    }
}
