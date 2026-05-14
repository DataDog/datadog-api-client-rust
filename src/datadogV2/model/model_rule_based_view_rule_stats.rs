// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Counts of findings for the rule, grouped by their evaluation status.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RuleBasedViewRuleStats {
    /// Number of findings that failed evaluation.
    #[serde(rename = "fail")]
    pub fail: i64,
    /// Number of findings that have been muted.
    #[serde(rename = "muted")]
    pub muted: i64,
    /// Number of findings that passed evaluation.
    #[serde(rename = "pass")]
    pub pass: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RuleBasedViewRuleStats {
    pub fn new(fail: i64, muted: i64, pass: i64) -> RuleBasedViewRuleStats {
        RuleBasedViewRuleStats {
            fail,
            muted,
            pass,
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

impl<'de> Deserialize<'de> for RuleBasedViewRuleStats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RuleBasedViewRuleStatsVisitor;
        impl<'a> Visitor<'a> for RuleBasedViewRuleStatsVisitor {
            type Value = RuleBasedViewRuleStats;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut fail: Option<i64> = None;
                let mut muted: Option<i64> = None;
                let mut pass: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "fail" => {
                            fail = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "muted" => {
                            muted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pass" => {
                            pass = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let fail = fail.ok_or_else(|| M::Error::missing_field("fail"))?;
                let muted = muted.ok_or_else(|| M::Error::missing_field("muted"))?;
                let pass = pass.ok_or_else(|| M::Error::missing_field("pass"))?;

                let content = RuleBasedViewRuleStats {
                    fail,
                    muted,
                    pass,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RuleBasedViewRuleStatsVisitor)
    }
}
