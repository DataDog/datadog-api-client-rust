// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of a scanning rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerRuleRelationships {
    /// A scanning group data.
    #[serde(rename = "group")]
    pub group: Option<crate::datadogV2::model::SensitiveDataScannerGroupData>,
    /// A standard pattern.
    #[serde(rename = "standard_pattern")]
    pub standard_pattern: Option<crate::datadogV2::model::SensitiveDataScannerStandardPatternData>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerRuleRelationships {
    pub fn new() -> SensitiveDataScannerRuleRelationships {
        SensitiveDataScannerRuleRelationships {
            group: None,
            standard_pattern: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn group(mut self, value: crate::datadogV2::model::SensitiveDataScannerGroupData) -> Self {
        self.group = Some(value);
        self
    }

    pub fn standard_pattern(
        mut self,
        value: crate::datadogV2::model::SensitiveDataScannerStandardPatternData,
    ) -> Self {
        self.standard_pattern = Some(value);
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

impl Default for SensitiveDataScannerRuleRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerRuleRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerRuleRelationshipsVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerRuleRelationshipsVisitor {
            type Value = SensitiveDataScannerRuleRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut group: Option<crate::datadogV2::model::SensitiveDataScannerGroupData> =
                    None;
                let mut standard_pattern: Option<
                    crate::datadogV2::model::SensitiveDataScannerStandardPatternData,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "group" => {
                            if v.is_null() {
                                continue;
                            }
                            group = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "standard_pattern" => {
                            if v.is_null() {
                                continue;
                            }
                            standard_pattern =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SensitiveDataScannerRuleRelationships {
                    group,
                    standard_pattern,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerRuleRelationshipsVisitor)
    }
}
