// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating an AWS cloud authentication intake mapping.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSCloudAuthIntakeMappingCreateAttributes {
    /// AWS caller ARN pattern allowed to authenticate for telemetry submission. ARN syntax and AWS integration membership are validated by the server. For an assumed role, use the STS assumed-role ARN with a trailing /* wildcard to match its sessions.
    #[serde(rename = "arn_pattern")]
    pub arn_pattern: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSCloudAuthIntakeMappingCreateAttributes {
    pub fn new(arn_pattern: String) -> AWSCloudAuthIntakeMappingCreateAttributes {
        AWSCloudAuthIntakeMappingCreateAttributes {
            arn_pattern,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for AWSCloudAuthIntakeMappingCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSCloudAuthIntakeMappingCreateAttributesVisitor;
        impl<'a> Visitor<'a> for AWSCloudAuthIntakeMappingCreateAttributesVisitor {
            type Value = AWSCloudAuthIntakeMappingCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut arn_pattern: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "arn_pattern" => {
                            arn_pattern =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let arn_pattern =
                    arn_pattern.ok_or_else(|| M::Error::missing_field("arn_pattern"))?;

                let content = AWSCloudAuthIntakeMappingCreateAttributes {
                    arn_pattern,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSCloudAuthIntakeMappingCreateAttributesVisitor)
    }
}
