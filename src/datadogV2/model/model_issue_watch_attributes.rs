// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the information of a watch.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueWatchAttributes {
    /// Action applied to the issue when the watch condition is met.
    #[serde(rename = "action")]
    pub action: Option<crate::datadogV2::model::IssueWatchAction>,
    /// Condition that triggers the watch action. The fields that are set depend on `kind`:
    /// `OCCURRENCES` uses `threshold`, `CARDINALITY` uses `threshold` and `projection`,
    /// `ELAPSED_TIME` uses `duration`, and `ANY` and `ALL` use `operands`.
    #[serde(rename = "condition")]
    pub condition: Option<crate::datadogV2::model::IssueWatchCondition>,
    /// Timestamp of when the watch started.
    #[serde(rename = "started_at")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Status of the watch.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::IssueWatchStatus>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueWatchAttributes {
    pub fn new() -> IssueWatchAttributes {
        IssueWatchAttributes {
            action: None,
            condition: None,
            started_at: None,
            status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn action(mut self, value: crate::datadogV2::model::IssueWatchAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn condition(mut self, value: crate::datadogV2::model::IssueWatchCondition) -> Self {
        self.condition = Some(value);
        self
    }

    pub fn started_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::IssueWatchStatus) -> Self {
        self.status = Some(value);
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

impl Default for IssueWatchAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IssueWatchAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueWatchAttributesVisitor;
        impl<'a> Visitor<'a> for IssueWatchAttributesVisitor {
            type Value = IssueWatchAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::IssueWatchAction> = None;
                let mut condition: Option<crate::datadogV2::model::IssueWatchCondition> = None;
                let mut started_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut status: Option<crate::datadogV2::model::IssueWatchStatus> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            if v.is_null() {
                                continue;
                            }
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "condition" => {
                            if v.is_null() {
                                continue;
                            }
                            condition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "started_at" => {
                            if v.is_null() {
                                continue;
                            }
                            started_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::IssueWatchStatus::UnparsedObject(
                                        _status,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IssueWatchAttributes {
                    action,
                    condition,
                    started_at,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueWatchAttributesVisitor)
    }
}
