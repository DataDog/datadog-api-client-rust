// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ApplicationSecurityWafCustomRuleActionParameters` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationSecurityWafCustomRuleActionParameters {
    /// The location to redirect to when the WAF custom rule triggers.
    #[serde(rename = "location")]
    pub location: Option<String>,
    /// The status code to return when the WAF custom rule triggers.
    #[serde(rename = "status_code")]
    pub status_code: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationSecurityWafCustomRuleActionParameters {
    pub fn new() -> ApplicationSecurityWafCustomRuleActionParameters {
        ApplicationSecurityWafCustomRuleActionParameters {
            location: None,
            status_code: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn location(mut self, value: String) -> Self {
        self.location = Some(value);
        self
    }

    pub fn status_code(mut self, value: i64) -> Self {
        self.status_code = Some(value);
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

impl Default for ApplicationSecurityWafCustomRuleActionParameters {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ApplicationSecurityWafCustomRuleActionParameters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationSecurityWafCustomRuleActionParametersVisitor;
        impl<'a> Visitor<'a> for ApplicationSecurityWafCustomRuleActionParametersVisitor {
            type Value = ApplicationSecurityWafCustomRuleActionParameters;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut location: Option<String> = None;
                let mut status_code: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "location" => {
                            if v.is_null() {
                                continue;
                            }
                            location = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status_code" => {
                            if v.is_null() {
                                continue;
                            }
                            status_code =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ApplicationSecurityWafCustomRuleActionParameters {
                    location,
                    status_code,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationSecurityWafCustomRuleActionParametersVisitor)
    }
}
