// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response for retrieving a partial application key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PartialApplicationKeyResponse {
    /// Partial Datadog application key.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::PartialApplicationKey>,
    /// Array of objects related to the application key.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::ApplicationKeyResponseIncludedItem>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PartialApplicationKeyResponse {
    pub fn new() -> PartialApplicationKeyResponse {
        PartialApplicationKeyResponse {
            data: None,
            included: None,
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: crate::datadogV2::model::PartialApplicationKey) -> Self {
        self.data = Some(value);
        self
    }

    pub fn included(
        mut self,
        value: Vec<crate::datadogV2::model::ApplicationKeyResponseIncludedItem>,
    ) -> Self {
        self.included = Some(value);
        self
    }
}

impl Default for PartialApplicationKeyResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for PartialApplicationKeyResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PartialApplicationKeyResponseVisitor;
        impl<'a> Visitor<'a> for PartialApplicationKeyResponseVisitor {
            type Value = PartialApplicationKeyResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::PartialApplicationKey> = None;
                let mut included: Option<
                    Vec<crate::datadogV2::model::ApplicationKeyResponseIncludedItem>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "included" => {
                            if v.is_null() {
                                continue;
                            }
                            included = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = PartialApplicationKeyResponse {
                    data,
                    included,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PartialApplicationKeyResponseVisitor)
    }
}
