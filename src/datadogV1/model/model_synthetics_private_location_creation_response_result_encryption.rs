// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Public key for the result encryption.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsPrivateLocationCreationResponseResultEncryption {
    /// Fingerprint for the encryption key.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Public key for result encryption.
    #[serde(rename = "key")]
    pub key: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsPrivateLocationCreationResponseResultEncryption {
    pub fn new() -> SyntheticsPrivateLocationCreationResponseResultEncryption {
        SyntheticsPrivateLocationCreationResponseResultEncryption {
            id: None,
            key: None,
            _unparsed: false,
        }
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn key(&mut self, value: String) -> &mut Self {
        self.key = Some(value);
        self
    }
}

impl Default for SyntheticsPrivateLocationCreationResponseResultEncryption {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsPrivateLocationCreationResponseResultEncryption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsPrivateLocationCreationResponseResultEncryptionVisitor;
        impl<'a> Visitor<'a> for SyntheticsPrivateLocationCreationResponseResultEncryptionVisitor {
            type Value = SyntheticsPrivateLocationCreationResponseResultEncryption;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut key: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            if v.is_null() {
                                continue;
                            }
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsPrivateLocationCreationResponseResultEncryption {
                    id,
                    key,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(SyntheticsPrivateLocationCreationResponseResultEncryptionVisitor)
    }
}
