// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Result of the convert rule request containing Terraform content.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleConvertResponse {
    /// Terraform string as a result of converting the rule from JSON.
    #[serde(rename = "terraformContent")]
    pub terraform_content: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleConvertResponse {
    pub fn new() -> SecurityMonitoringRuleConvertResponse {
        SecurityMonitoringRuleConvertResponse {
            terraform_content: None,
            _unparsed: false,
        }
    }

    pub fn terraform_content(mut self, value: String) -> Self {
        self.terraform_content = Some(value);
        self
    }
}

impl Default for SecurityMonitoringRuleConvertResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleConvertResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleConvertResponseVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleConvertResponseVisitor {
            type Value = SecurityMonitoringRuleConvertResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut terraform_content: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "terraformContent" => {
                            if v.is_null() {
                                continue;
                            }
                            terraform_content =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SecurityMonitoringRuleConvertResponse {
                    terraform_content,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleConvertResponseVisitor)
    }
}
