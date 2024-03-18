// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing information about the IP allowlist.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IPAllowlistResponse {
    /// IP allowlist data.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::IPAllowlistData>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IPAllowlistResponse {
    pub fn new() -> IPAllowlistResponse {
        IPAllowlistResponse {
            data: None,
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: crate::datadogV2::model::IPAllowlistData) -> Self {
        self.data = Some(value);
        self
    }
}

impl Default for IPAllowlistResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IPAllowlistResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IPAllowlistResponseVisitor;
        impl<'a> Visitor<'a> for IPAllowlistResponseVisitor {
            type Value = IPAllowlistResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::IPAllowlistData> = None;
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

                let content = IPAllowlistResponse { data, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IPAllowlistResponseVisitor)
    }
}
