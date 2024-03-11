// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response for retrieving an application key.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationKeyResponse {
    /// Datadog application key.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::FullApplicationKey>,
    /// Array of objects related to the application key.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::ApplicationKeyResponseIncludedItem>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationKeyResponse {
    pub fn new() -> ApplicationKeyResponse {
        ApplicationKeyResponse {
            data: None,
            included: None,
            _unparsed: false,
        }
    }

    pub fn data(&mut self, value: crate::datadogV2::model::FullApplicationKey) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn included(
        &mut self,
        value: Vec<crate::datadogV2::model::ApplicationKeyResponseIncludedItem>,
    ) -> &mut Self {
        self.included = Some(value);
        self
    }
}

impl Default for ApplicationKeyResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ApplicationKeyResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationKeyResponseVisitor;
        impl<'a> Visitor<'a> for ApplicationKeyResponseVisitor {
            type Value = ApplicationKeyResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::FullApplicationKey> = None;
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

                let content = ApplicationKeyResponse {
                    data,
                    included,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationKeyResponseVisitor)
    }
}
