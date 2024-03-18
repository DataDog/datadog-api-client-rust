// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A JSON list of the ID or IDs of the Synthetic tests that you want
/// to delete.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsDeleteTestsPayload {
    /// An array of Synthetic test IDs you want to delete.
    #[serde(rename = "public_ids")]
    pub public_ids: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsDeleteTestsPayload {
    pub fn new() -> SyntheticsDeleteTestsPayload {
        SyntheticsDeleteTestsPayload {
            public_ids: None,
            _unparsed: false,
        }
    }

    pub fn public_ids(mut self, value: Vec<String>) -> Self {
        self.public_ids = Some(value);
        self
    }
}

impl Default for SyntheticsDeleteTestsPayload {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsDeleteTestsPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsDeleteTestsPayloadVisitor;
        impl<'a> Visitor<'a> for SyntheticsDeleteTestsPayloadVisitor {
            type Value = SyntheticsDeleteTestsPayload;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut public_ids: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "public_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            public_ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsDeleteTestsPayload {
                    public_ids,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsDeleteTestsPayloadVisitor)
    }
}
