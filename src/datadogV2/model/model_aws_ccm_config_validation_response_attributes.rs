// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for an AWS CCM config validation response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSCcmConfigValidationResponseAttributes {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// List of validation issues found for the Cost and Usage Report (CUR) 2.0 configuration. Empty when the configuration is valid.
    #[serde(rename = "issues")]
    pub issues: Vec<crate::datadogV2::model::AWSCcmConfigValidationIssue>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSCcmConfigValidationResponseAttributes {
    pub fn new(
        account_id: String,
        issues: Vec<crate::datadogV2::model::AWSCcmConfigValidationIssue>,
    ) -> AWSCcmConfigValidationResponseAttributes {
        AWSCcmConfigValidationResponseAttributes {
            account_id,
            issues,
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

impl<'de> Deserialize<'de> for AWSCcmConfigValidationResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSCcmConfigValidationResponseAttributesVisitor;
        impl<'a> Visitor<'a> for AWSCcmConfigValidationResponseAttributesVisitor {
            type Value = AWSCcmConfigValidationResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut issues: Option<Vec<crate::datadogV2::model::AWSCcmConfigValidationIssue>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issues" => {
                            issues = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let account_id = account_id.ok_or_else(|| M::Error::missing_field("account_id"))?;
                let issues = issues.ok_or_else(|| M::Error::missing_field("issues"))?;

                let content = AWSCcmConfigValidationResponseAttributes {
                    account_id,
                    issues,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSCcmConfigValidationResponseAttributesVisitor)
    }
}
