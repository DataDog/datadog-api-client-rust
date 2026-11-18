// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of a rule of a scanning group of a target.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerTargetRuleRelationships {
    /// A scanning group data.
    #[serde(rename = "group")]
    pub group: Option<crate::datadogV2::model::SensitiveDataScannerGroupData>,
    /// The standard rule a rule is based on. Not included for custom rules.
    #[serde(rename = "standard_rule")]
    pub standard_rule: Option<crate::datadogV2::model::SensitiveDataScannerStandardRule>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerTargetRuleRelationships {
    pub fn new() -> SensitiveDataScannerTargetRuleRelationships {
        SensitiveDataScannerTargetRuleRelationships {
            group: None,
            standard_rule: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn group(mut self, value: crate::datadogV2::model::SensitiveDataScannerGroupData) -> Self {
        self.group = Some(value);
        self
    }

    pub fn standard_rule(
        mut self,
        value: crate::datadogV2::model::SensitiveDataScannerStandardRule,
    ) -> Self {
        self.standard_rule = Some(value);
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

impl Default for SensitiveDataScannerTargetRuleRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerTargetRuleRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerTargetRuleRelationshipsVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerTargetRuleRelationshipsVisitor {
            type Value = SensitiveDataScannerTargetRuleRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut group: Option<crate::datadogV2::model::SensitiveDataScannerGroupData> =
                    None;
                let mut standard_rule: Option<
                    crate::datadogV2::model::SensitiveDataScannerStandardRule,
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
                        "standard_rule" => {
                            if v.is_null() {
                                continue;
                            }
                            standard_rule =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SensitiveDataScannerTargetRuleRelationships {
                    group,
                    standard_rule,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerTargetRuleRelationshipsVisitor)
    }
}
