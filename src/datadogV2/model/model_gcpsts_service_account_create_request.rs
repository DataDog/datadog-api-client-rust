// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data on your newly generated service account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GCPSTSServiceAccountCreateRequest {
    /// Additional metadata on your generated service account.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::GCPSTSServiceAccountData>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GCPSTSServiceAccountCreateRequest {
    pub fn new() -> GCPSTSServiceAccountCreateRequest {
        GCPSTSServiceAccountCreateRequest {
            data: None,
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: crate::datadogV2::model::GCPSTSServiceAccountData) -> Self {
        self.data = Some(value);
        self
    }
}

impl Default for GCPSTSServiceAccountCreateRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GCPSTSServiceAccountCreateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GCPSTSServiceAccountCreateRequestVisitor;
        impl<'a> Visitor<'a> for GCPSTSServiceAccountCreateRequestVisitor {
            type Value = GCPSTSServiceAccountCreateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::GCPSTSServiceAccountData> = None;
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

                let content = GCPSTSServiceAccountCreateRequest { data, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GCPSTSServiceAccountCreateRequestVisitor)
    }
}
