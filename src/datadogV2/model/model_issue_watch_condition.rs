// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Condition that triggers the watch action. The fields that are set depend on `kind`:
/// `OCCURRENCES` uses `threshold`, `CARDINALITY` uses `threshold` and `projection`,
/// `ELAPSED_TIME` uses `duration`, and `ANY` and `ALL` use `operands`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueWatchCondition {
    /// Time that must elapse before the condition is met, as a Go duration string.
    #[serde(rename = "duration")]
    pub duration: Option<String>,
    /// Kind of watch condition.
    #[serde(rename = "kind")]
    pub kind: crate::datadogV2::model::IssueWatchConditionKind,
    /// Nested conditions combined by `ANY` or `ALL`.
    #[serde(rename = "operands")]
    pub operands: Option<Vec<crate::datadogV2::model::IssueWatchCondition>>,
    /// Attributes whose distinct values are counted for `CARDINALITY`.
    #[serde(rename = "projection")]
    pub projection: Option<Vec<String>>,
    /// Number of occurrences, or of distinct values for `CARDINALITY`, that triggers the condition.
    #[serde(rename = "threshold")]
    pub threshold: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueWatchCondition {
    pub fn new(kind: crate::datadogV2::model::IssueWatchConditionKind) -> IssueWatchCondition {
        IssueWatchCondition {
            duration: None,
            kind,
            operands: None,
            projection: None,
            threshold: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn duration(mut self, value: String) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn operands(mut self, value: Vec<crate::datadogV2::model::IssueWatchCondition>) -> Self {
        self.operands = Some(value);
        self
    }

    pub fn projection(mut self, value: Vec<String>) -> Self {
        self.projection = Some(value);
        self
    }

    pub fn threshold(mut self, value: i64) -> Self {
        self.threshold = Some(value);
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

impl<'de> Deserialize<'de> for IssueWatchCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueWatchConditionVisitor;
        impl<'a> Visitor<'a> for IssueWatchConditionVisitor {
            type Value = IssueWatchCondition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut duration: Option<String> = None;
                let mut kind: Option<crate::datadogV2::model::IssueWatchConditionKind> = None;
                let mut operands: Option<Vec<crate::datadogV2::model::IssueWatchCondition>> = None;
                let mut projection: Option<Vec<String>> = None;
                let mut threshold: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "duration" => {
                            if v.is_null() {
                                continue;
                            }
                            duration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "kind" => {
                            kind = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _kind) = kind {
                                match _kind {
                                    crate::datadogV2::model::IssueWatchConditionKind::UnparsedObject(_kind) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "operands" => {
                            if v.is_null() {
                                continue;
                            }
                            operands = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "projection" => {
                            if v.is_null() {
                                continue;
                            }
                            projection = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "threshold" => {
                            if v.is_null() {
                                continue;
                            }
                            threshold = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let kind = kind.ok_or_else(|| M::Error::missing_field("kind"))?;

                let content = IssueWatchCondition {
                    duration,
                    kind,
                    operands,
                    projection,
                    threshold,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueWatchConditionVisitor)
    }
}
