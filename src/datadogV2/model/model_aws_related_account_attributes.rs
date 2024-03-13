// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for an AWS related account.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSRelatedAccountAttributes {
    /// Whether or not the AWS account has a Datadog integration.
    #[serde(rename = "has_datadog_integration")]
    pub has_datadog_integration: Option<bool>,
    /// The name of the AWS account.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSRelatedAccountAttributes {
    pub fn new() -> AWSRelatedAccountAttributes {
        AWSRelatedAccountAttributes {
            has_datadog_integration: None,
            name: None,
            _unparsed: false,
        }
    }

    pub fn has_datadog_integration(mut self, value: bool) -> Self {
        self.has_datadog_integration = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
}

impl Default for AWSRelatedAccountAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSRelatedAccountAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSRelatedAccountAttributesVisitor;
        impl<'a> Visitor<'a> for AWSRelatedAccountAttributesVisitor {
            type Value = AWSRelatedAccountAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut has_datadog_integration: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "has_datadog_integration" => {
                            if v.is_null() {
                                continue;
                            }
                            has_datadog_integration =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSRelatedAccountAttributes {
                    has_datadog_integration,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSRelatedAccountAttributesVisitor)
    }
}
