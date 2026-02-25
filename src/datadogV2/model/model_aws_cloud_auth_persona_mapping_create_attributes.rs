// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating an AWS cloud authentication persona mapping
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSCloudAuthPersonaMappingCreateAttributes {
    /// Datadog account identifier (email or handle) mapped to the AWS principal
    #[serde(rename = "account_identifier")]
    pub account_identifier: String,
    /// AWS IAM ARN pattern to match for authentication
    #[serde(rename = "arn_pattern")]
    pub arn_pattern: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSCloudAuthPersonaMappingCreateAttributes {
    pub fn new(
        account_identifier: String,
        arn_pattern: String,
    ) -> AWSCloudAuthPersonaMappingCreateAttributes {
        AWSCloudAuthPersonaMappingCreateAttributes {
            account_identifier,
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

impl<'de> Deserialize<'de> for AWSCloudAuthPersonaMappingCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSCloudAuthPersonaMappingCreateAttributesVisitor;
        impl<'a> Visitor<'a> for AWSCloudAuthPersonaMappingCreateAttributesVisitor {
            type Value = AWSCloudAuthPersonaMappingCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_identifier: Option<String> = None;
                let mut arn_pattern: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_identifier" => {
                            account_identifier =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                let account_identifier = account_identifier
                    .ok_or_else(|| M::Error::missing_field("account_identifier"))?;
                let arn_pattern =
                    arn_pattern.ok_or_else(|| M::Error::missing_field("arn_pattern"))?;

                let content = AWSCloudAuthPersonaMappingCreateAttributes {
                    account_identifier,
                    arn_pattern,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSCloudAuthPersonaMappingCreateAttributesVisitor)
    }
}
