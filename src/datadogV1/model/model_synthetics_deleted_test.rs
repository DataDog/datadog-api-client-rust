// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing a deleted Synthetic test ID with the associated
/// deletion timestamp.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsDeletedTest {
    /// Deletion timestamp of the Synthetic test ID.
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The Synthetic test ID deleted.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsDeletedTest {
    pub fn new() -> SyntheticsDeletedTest {
        SyntheticsDeletedTest {
            deleted_at: None,
            public_id: None,
            _unparsed: false,
        }
    }

    pub fn deleted_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }
}

impl Default for SyntheticsDeletedTest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsDeletedTest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsDeletedTestVisitor;
        impl<'a> Visitor<'a> for SyntheticsDeletedTestVisitor {
            type Value = SyntheticsDeletedTest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut deleted_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut public_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "deleted_at" => {
                            if v.is_null() {
                                continue;
                            }
                            deleted_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsDeletedTest {
                    deleted_at,
                    public_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsDeletedTestVisitor)
    }
}
