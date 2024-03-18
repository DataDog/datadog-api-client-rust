// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A service level objective object includes a service level indicator, thresholds
/// for one or more timeframes, and metadata (`name`, `description`, `tags`, etc.).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceLevelObjectiveRequest {
    /// A user-defined description of the service level objective.
    ///
    /// Always included in service level objective responses (but may be `null`).
    /// Optional in create/update requests.
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    /// A list of (up to 100) monitor groups that narrow the scope of a monitor service level objective.
    ///
    /// Included in service level objective responses if it is not empty. Optional in
    /// create/update requests for monitor service level objectives, but may only be
    /// used when then length of the `monitor_ids` field is one.
    #[serde(rename = "groups")]
    pub groups: Option<Vec<String>>,
    /// A list of monitor IDs that defines the scope of a monitor service level
    /// objective. **Required if type is `monitor`**.
    #[serde(rename = "monitor_ids")]
    pub monitor_ids: Option<Vec<i64>>,
    /// The name of the service level objective object.
    #[serde(rename = "name")]
    pub name: String,
    /// A metric-based SLO. **Required if type is `metric`**. Note that Datadog only allows the sum by aggregator
    /// to be used because this will sum up all request counts instead of averaging them, or taking the max or
    /// min of all of those requests.
    #[serde(rename = "query")]
    pub query: Option<crate::datadogV1::model::ServiceLevelObjectiveQuery>,
    /// A list of tags associated with this service level objective.
    /// Always included in service level objective responses (but may be empty).
    /// Optional in create/update requests.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The target threshold such that when the service level indicator is above this
    /// threshold over the given timeframe, the objective is being met.
    #[serde(rename = "target_threshold")]
    pub target_threshold: Option<f64>,
    /// The thresholds (timeframes and associated targets) for this service level
    /// objective object.
    #[serde(rename = "thresholds")]
    pub thresholds: Vec<crate::datadogV1::model::SLOThreshold>,
    /// The SLO time window options.
    #[serde(rename = "timeframe")]
    pub timeframe: Option<crate::datadogV1::model::SLOTimeframe>,
    /// The type of the service level objective.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SLOType,
    /// The optional warning threshold such that when the service level indicator is
    /// below this value for the given threshold, but above the target threshold, the
    /// objective appears in a "warning" state. This value must be greater than the target
    /// threshold.
    #[serde(rename = "warning_threshold")]
    pub warning_threshold: Option<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceLevelObjectiveRequest {
    pub fn new(
        name: String,
        thresholds: Vec<crate::datadogV1::model::SLOThreshold>,
        type_: crate::datadogV1::model::SLOType,
    ) -> ServiceLevelObjectiveRequest {
        ServiceLevelObjectiveRequest {
            description: None,
            groups: None,
            monitor_ids: None,
            name,
            query: None,
            tags: None,
            target_threshold: None,
            thresholds,
            timeframe: None,
            type_,
            warning_threshold: None,
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn groups(mut self, value: Vec<String>) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn monitor_ids(mut self, value: Vec<i64>) -> Self {
        self.monitor_ids = Some(value);
        self
    }

    pub fn query(mut self, value: crate::datadogV1::model::ServiceLevelObjectiveQuery) -> Self {
        self.query = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn target_threshold(mut self, value: f64) -> Self {
        self.target_threshold = Some(value);
        self
    }

    pub fn timeframe(mut self, value: crate::datadogV1::model::SLOTimeframe) -> Self {
        self.timeframe = Some(value);
        self
    }

    pub fn warning_threshold(mut self, value: f64) -> Self {
        self.warning_threshold = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ServiceLevelObjectiveRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceLevelObjectiveRequestVisitor;
        impl<'a> Visitor<'a> for ServiceLevelObjectiveRequestVisitor {
            type Value = ServiceLevelObjectiveRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<Option<String>> = None;
                let mut groups: Option<Vec<String>> = None;
                let mut monitor_ids: Option<Vec<i64>> = None;
                let mut name: Option<String> = None;
                let mut query: Option<crate::datadogV1::model::ServiceLevelObjectiveQuery> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut target_threshold: Option<f64> = None;
                let mut thresholds: Option<Vec<crate::datadogV1::model::SLOThreshold>> = None;
                let mut timeframe: Option<crate::datadogV1::model::SLOTimeframe> = None;
                let mut type_: Option<crate::datadogV1::model::SLOType> = None;
                let mut warning_threshold: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "groups" => {
                            if v.is_null() {
                                continue;
                            }
                            groups = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_threshold" => {
                            if v.is_null() {
                                continue;
                            }
                            target_threshold =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "thresholds" => {
                            thresholds = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeframe" => {
                            if v.is_null() {
                                continue;
                            }
                            timeframe = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _timeframe) = timeframe {
                                match _timeframe {
                                    crate::datadogV1::model::SLOTimeframe::UnparsedObject(
                                        _timeframe,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SLOType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "warning_threshold" => {
                            if v.is_null() {
                                continue;
                            }
                            warning_threshold =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let thresholds = thresholds.ok_or_else(|| M::Error::missing_field("thresholds"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ServiceLevelObjectiveRequest {
                    description,
                    groups,
                    monitor_ids,
                    name,
                    query,
                    tags,
                    target_threshold,
                    thresholds,
                    timeframe,
                    type_,
                    warning_threshold,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceLevelObjectiveRequestVisitor)
    }
}
