// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Target WAF rules based either on an identifier or tags.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationSecurityWafExclusionFilterRulesTarget {
    /// Target a single WAF rule based on its identifier.
    #[serde(rename = "rule_id")]
    pub rule_id: Option<String>,
    /// Target multiple WAF rules based on their tags.
    #[serde(rename = "tags")]
    pub tags: Option<crate::datadogV2::model::ApplicationSecurityWafExclusionFilterRulesTargetTags>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationSecurityWafExclusionFilterRulesTarget {
    pub fn new() -> ApplicationSecurityWafExclusionFilterRulesTarget {
        ApplicationSecurityWafExclusionFilterRulesTarget {
            rule_id: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn rule_id(mut self, value: String) -> Self {
        self.rule_id = Some(value);
        self
    }

    pub fn tags(
        mut self,
        value: crate::datadogV2::model::ApplicationSecurityWafExclusionFilterRulesTargetTags,
    ) -> Self {
        self.tags = Some(value);
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

impl Default for ApplicationSecurityWafExclusionFilterRulesTarget {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ApplicationSecurityWafExclusionFilterRulesTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationSecurityWafExclusionFilterRulesTargetVisitor;
        impl<'a> Visitor<'a> for ApplicationSecurityWafExclusionFilterRulesTargetVisitor {
            type Value = ApplicationSecurityWafExclusionFilterRulesTarget;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut rule_id: Option<String> = None;
                let mut tags: Option<
                    crate::datadogV2::model::ApplicationSecurityWafExclusionFilterRulesTargetTags,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "rule_id" => {
                            if v.is_null() {
                                continue;
                            }
                            rule_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ApplicationSecurityWafExclusionFilterRulesTarget {
                    rule_id,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationSecurityWafExclusionFilterRulesTargetVisitor)
    }
}
