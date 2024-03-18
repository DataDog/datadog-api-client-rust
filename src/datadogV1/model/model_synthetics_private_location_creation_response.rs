// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object that contains the new private location, the public key for result encryption, and the configuration skeleton.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsPrivateLocationCreationResponse {
    /// Configuration skeleton for the private location. See installation instructions of the private location on how to use this configuration.
    #[serde(rename = "config")]
    pub config: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Object containing information about the private location to create.
    #[serde(rename = "private_location")]
    pub private_location: Option<crate::datadogV1::model::SyntheticsPrivateLocation>,
    /// Public key for the result encryption.
    #[serde(rename = "result_encryption")]
    pub result_encryption:
        Option<crate::datadogV1::model::SyntheticsPrivateLocationCreationResponseResultEncryption>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsPrivateLocationCreationResponse {
    pub fn new() -> SyntheticsPrivateLocationCreationResponse {
        SyntheticsPrivateLocationCreationResponse {
            config: None,
            private_location: None,
            result_encryption: None,
            _unparsed: false,
        }
    }

    pub fn config(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.config = Some(value);
        self
    }

    pub fn private_location(
        mut self,
        value: crate::datadogV1::model::SyntheticsPrivateLocation,
    ) -> Self {
        self.private_location = Some(value);
        self
    }

    pub fn result_encryption(
        mut self,
        value: crate::datadogV1::model::SyntheticsPrivateLocationCreationResponseResultEncryption,
    ) -> Self {
        self.result_encryption = Some(value);
        self
    }
}

impl Default for SyntheticsPrivateLocationCreationResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsPrivateLocationCreationResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsPrivateLocationCreationResponseVisitor;
        impl<'a> Visitor<'a> for SyntheticsPrivateLocationCreationResponseVisitor {
            type Value = SyntheticsPrivateLocationCreationResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut config: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut private_location: Option<
                    crate::datadogV1::model::SyntheticsPrivateLocation,
                > = None;
                let mut result_encryption: Option<crate::datadogV1::model::SyntheticsPrivateLocationCreationResponseResultEncryption> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "config" => {
                            if v.is_null() {
                                continue;
                            }
                            config = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "private_location" => {
                            if v.is_null() {
                                continue;
                            }
                            private_location =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "result_encryption" => {
                            if v.is_null() {
                                continue;
                            }
                            result_encryption =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsPrivateLocationCreationResponse {
                    config,
                    private_location,
                    result_encryption,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsPrivateLocationCreationResponseVisitor)
    }
}
