// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details about a batch response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsBatchDetails {
    /// Wrapper object that contains the details of a batch.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV1::model::SyntheticsBatchDetailsData>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsBatchDetails {
    pub fn new() -> SyntheticsBatchDetails {
        SyntheticsBatchDetails {
            data: None,
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: crate::datadogV1::model::SyntheticsBatchDetailsData) -> Self {
        self.data = Some(value);
        self
    }
}

impl Default for SyntheticsBatchDetails {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsBatchDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBatchDetailsVisitor;
        impl<'a> Visitor<'a> for SyntheticsBatchDetailsVisitor {
            type Value = SyntheticsBatchDetails;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV1::model::SyntheticsBatchDetailsData> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsBatchDetails { data, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBatchDetailsVisitor)
    }
}
