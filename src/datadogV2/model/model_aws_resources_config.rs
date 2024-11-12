// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS Resources config
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSResourcesConfig {
    /// Whether Datadog collects cloud security posture management resources from your AWS account.
    #[serde(rename = "cloud_security_posture_management_collection")]
    pub cloud_security_posture_management_collection: Option<bool>,
    /// Whether Datadog collects additional attributes and configuration information about the resources in your AWS account. Required for `cspm_resource_collection`.
    #[serde(rename = "extended_collection")]
    pub extended_collection: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSResourcesConfig {
    pub fn new() -> AWSResourcesConfig {
        AWSResourcesConfig {
            cloud_security_posture_management_collection: None,
            extended_collection: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cloud_security_posture_management_collection(mut self, value: bool) -> Self {
        self.cloud_security_posture_management_collection = Some(value);
        self
    }

    pub fn extended_collection(mut self, value: bool) -> Self {
        self.extended_collection = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for AWSResourcesConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSResourcesConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSResourcesConfigVisitor;
        impl<'a> Visitor<'a> for AWSResourcesConfigVisitor {
            type Value = AWSResourcesConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cloud_security_posture_management_collection: Option<bool> = None;
                let mut extended_collection: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cloud_security_posture_management_collection" => {
                            if v.is_null() {
                                continue;
                            }
                            cloud_security_posture_management_collection =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "extended_collection" => {
                            if v.is_null() {
                                continue;
                            }
                            extended_collection =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AWSResourcesConfig {
                    cloud_security_posture_management_collection,
                    extended_collection,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSResourcesConfigVisitor)
    }
}
