// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing the suppressions for a rule. There are three types of suppressions, `starts_with`, `ends_with`, and `exact_match`.
/// Suppressed matches are not obfuscated, counted in metrics, or displayed in the Findings page.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerSuppressions {
    /// List of strings to use for suppression of matches ending with these strings.
    #[serde(rename = "ends_with")]
    pub ends_with: Option<Vec<String>>,
    /// List of strings to use for suppression of matches exactly matching these strings.
    #[serde(rename = "exact_match")]
    pub exact_match: Option<Vec<String>>,
    /// List of strings to use for suppression of matches starting with these strings.
    #[serde(rename = "starts_with")]
    pub starts_with: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerSuppressions {
    pub fn new() -> SensitiveDataScannerSuppressions {
        SensitiveDataScannerSuppressions {
            ends_with: None,
            exact_match: None,
            starts_with: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn ends_with(mut self, value: Vec<String>) -> Self {
        self.ends_with = Some(value);
        self
    }

    pub fn exact_match(mut self, value: Vec<String>) -> Self {
        self.exact_match = Some(value);
        self
    }

    pub fn starts_with(mut self, value: Vec<String>) -> Self {
        self.starts_with = Some(value);
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

impl Default for SensitiveDataScannerSuppressions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerSuppressions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerSuppressionsVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerSuppressionsVisitor {
            type Value = SensitiveDataScannerSuppressions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ends_with: Option<Vec<String>> = None;
                let mut exact_match: Option<Vec<String>> = None;
                let mut starts_with: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ends_with" => {
                            if v.is_null() {
                                continue;
                            }
                            ends_with = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exact_match" => {
                            if v.is_null() {
                                continue;
                            }
                            exact_match =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "starts_with" => {
                            if v.is_null() {
                                continue;
                            }
                            starts_with =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SensitiveDataScannerSuppressions {
                    ends_with,
                    exact_match,
                    starts_with,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerSuppressionsVisitor)
    }
}
