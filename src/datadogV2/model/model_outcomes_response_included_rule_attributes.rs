// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Details of a rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OutcomesResponseIncludedRuleAttributes {
    /// Name of the rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The scorecard name to which this rule must belong.
    #[serde(rename = "scorecard_name")]
    pub scorecard_name: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OutcomesResponseIncludedRuleAttributes {
    pub fn new() -> OutcomesResponseIncludedRuleAttributes {
        OutcomesResponseIncludedRuleAttributes {
            name: None,
            scorecard_name: None,
            _unparsed: false,
        }
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn scorecard_name(mut self, value: String) -> Self {
        self.scorecard_name = Some(value);
        self
    }
}

impl Default for OutcomesResponseIncludedRuleAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OutcomesResponseIncludedRuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OutcomesResponseIncludedRuleAttributesVisitor;
        impl<'a> Visitor<'a> for OutcomesResponseIncludedRuleAttributesVisitor {
            type Value = OutcomesResponseIncludedRuleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut scorecard_name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scorecard_name" => {
                            if v.is_null() {
                                continue;
                            }
                            scorecard_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = OutcomesResponseIncludedRuleAttributes {
                    name,
                    scorecard_name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OutcomesResponseIncludedRuleAttributesVisitor)
    }
}
