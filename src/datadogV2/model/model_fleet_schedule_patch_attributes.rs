// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for partially updating a schedule. All fields are optional.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetSchedulePatchAttributes {
    /// Human-readable name for the schedule.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Query used to filter and select target hosts for scheduled deployments. Uses the Datadog query syntax.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// Defines the recurrence pattern for the schedule. Specifies when deployments should be
    /// automatically triggered based on maintenance windows.
    #[serde(rename = "rule")]
    pub rule: Option<crate::datadogV2::model::FleetScheduleRecurrenceRule>,
    /// The status of the schedule.
    /// - `active`: The schedule is active and will create deployments according to its recurrence rule.
    /// - `inactive`: The schedule is inactive and will not create any deployments.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::FleetScheduleStatus>,
    /// Number of major versions behind the latest to target for upgrades.
    /// - 0: Always upgrade to the latest version
    /// - 1: Upgrade to latest minus 1 major version
    /// - 2: Upgrade to latest minus 2 major versions
    /// Maximum value is 2.
    #[serde(rename = "version_to_latest")]
    pub version_to_latest: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetSchedulePatchAttributes {
    pub fn new() -> FleetSchedulePatchAttributes {
        FleetSchedulePatchAttributes {
            name: None,
            query: None,
            rule: None,
            status: None,
            version_to_latest: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn rule(mut self, value: crate::datadogV2::model::FleetScheduleRecurrenceRule) -> Self {
        self.rule = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::FleetScheduleStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn version_to_latest(mut self, value: i64) -> Self {
        self.version_to_latest = Some(value);
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

impl Default for FleetSchedulePatchAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FleetSchedulePatchAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetSchedulePatchAttributesVisitor;
        impl<'a> Visitor<'a> for FleetSchedulePatchAttributesVisitor {
            type Value = FleetSchedulePatchAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut rule: Option<crate::datadogV2::model::FleetScheduleRecurrenceRule> = None;
                let mut status: Option<crate::datadogV2::model::FleetScheduleStatus> = None;
                let mut version_to_latest: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule" => {
                            if v.is_null() {
                                continue;
                            }
                            rule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::FleetScheduleStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "version_to_latest" => {
                            if v.is_null() {
                                continue;
                            }
                            version_to_latest =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FleetSchedulePatchAttributes {
                    name,
                    query,
                    rule,
                    status,
                    version_to_latest,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetSchedulePatchAttributesVisitor)
    }
}
