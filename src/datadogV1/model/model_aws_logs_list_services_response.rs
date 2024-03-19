// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The list of current AWS services for which Datadog offers automatic log collection.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSLogsListServicesResponse {
    /// Key value in returned object.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Name of service available for configuration with Datadog logs.
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSLogsListServicesResponse {
    pub fn new() -> AWSLogsListServicesResponse {
        AWSLogsListServicesResponse {
            id: None,
            label: None,
            _unparsed: false,
        }
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
}

impl Default for AWSLogsListServicesResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSLogsListServicesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSLogsListServicesResponseVisitor;
        impl<'a> Visitor<'a> for AWSLogsListServicesResponseVisitor {
            type Value = AWSLogsListServicesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut label: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "label" => {
                            if v.is_null() {
                                continue;
                            }
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSLogsListServicesResponse {
                    id,
                    label,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSLogsListServicesResponseVisitor)
    }
}
