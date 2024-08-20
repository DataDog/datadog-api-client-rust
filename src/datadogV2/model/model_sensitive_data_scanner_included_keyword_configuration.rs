// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object defining a set of keywords and a number of characters that help reduce noise.
/// You can provide a list of keywords you would like to check within a defined proximity of the matching pattern.
/// If any of the keywords are found within the proximity check, the match is kept.
/// If none are found, the match is discarded.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerIncludedKeywordConfiguration {
    /// The number of characters behind a match detected by Sensitive Data Scanner to look for the keywords defined.
    /// `character_count` should be greater than the maximum length of a keyword defined for a rule.
    #[serde(rename = "character_count")]
    pub character_count: i64,
    /// Keyword list that will be checked during scanning in order to validate a match.
    /// The number of keywords in the list must be less than or equal to 30.
    #[serde(rename = "keywords")]
    pub keywords: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerIncludedKeywordConfiguration {
    pub fn new(
        character_count: i64,
        keywords: Vec<String>,
    ) -> SensitiveDataScannerIncludedKeywordConfiguration {
        SensitiveDataScannerIncludedKeywordConfiguration {
            character_count,
            keywords,
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

impl<'de> Deserialize<'de> for SensitiveDataScannerIncludedKeywordConfiguration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerIncludedKeywordConfigurationVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerIncludedKeywordConfigurationVisitor {
            type Value = SensitiveDataScannerIncludedKeywordConfiguration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut character_count: Option<i64> = None;
                let mut keywords: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "character_count" => {
                            character_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "keywords" => {
                            keywords = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let character_count =
                    character_count.ok_or_else(|| M::Error::missing_field("character_count"))?;
                let keywords = keywords.ok_or_else(|| M::Error::missing_field("keywords"))?;

                let content = SensitiveDataScannerIncludedKeywordConfiguration {
                    character_count,
                    keywords,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerIncludedKeywordConfigurationVisitor)
    }
}
