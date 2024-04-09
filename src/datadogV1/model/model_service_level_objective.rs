// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A service level objective object includes a service level indicator, thresholds
/// for one or more timeframes, and metadata (`name`, `description`, `tags`, etc.).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceLevelObjective {
    /// Creation timestamp (UNIX time in seconds)
    ///
    /// Always included in service level objective responses.
    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,
    /// Object describing the creator of the shared element.
    #[serde(rename = "creator")]
    pub creator: Option<crate::datadogV1::model::Creator>,
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
    /// A unique identifier for the service level objective object.
    ///
    /// Always included in service level objective responses.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Modification timestamp (UNIX time in seconds)
    ///
    /// Always included in service level objective responses.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<i64>,
    /// A list of monitor ids that defines the scope of a monitor service level
    /// objective. **Required if type is `monitor`**.
    #[serde(rename = "monitor_ids")]
    pub monitor_ids: Option<Vec<i64>>,
    /// The union of monitor tags for all monitors referenced by the `monitor_ids`
    /// field.
    /// Always included in service level objective responses for monitor-based service level
    /// objectives (but may be empty). Ignored in create/update requests. Does not
    /// affect which monitors are included in the service level objective (that is
    /// determined entirely by the `monitor_ids` field).
    #[serde(rename = "monitor_tags")]
    pub monitor_tags: Option<Vec<String>>,
    /// The name of the service level objective object.
    #[serde(rename = "name")]
    pub name: String,
    /// A metric-based SLO. **Required if type is `metric`**. Note that Datadog only allows the sum by aggregator
    /// to be used because this will sum up all request counts instead of averaging them, or taking the max or
    /// min of all of those requests.
    #[serde(rename = "query")]
    pub query: Option<crate::datadogV1::model::ServiceLevelObjectiveQuery>,
    /// A generic SLI specification. This is currently used for time-slice SLOs only.
    #[serde(rename = "sli_specification")]
    pub sli_specification: Option<crate::datadogV1::model::SLOSliSpec>,
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

impl ServiceLevelObjective {
    pub fn new(
        name: String,
        thresholds: Vec<crate::datadogV1::model::SLOThreshold>,
        type_: crate::datadogV1::model::SLOType,
    ) -> ServiceLevelObjective {
        ServiceLevelObjective {
            created_at: None,
            creator: None,
            description: None,
            groups: None,
            id: None,
            modified_at: None,
            monitor_ids: None,
            monitor_tags: None,
            name,
            query: None,
            sli_specification: None,
            tags: None,
            target_threshold: None,
            thresholds,
            timeframe: None,
            type_,
            warning_threshold: None,
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn creator(mut self, value: crate::datadogV1::model::Creator) -> Self {
        self.creator = Some(value);
        self
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn groups(mut self, value: Vec<String>) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn modified_at(mut self, value: i64) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn monitor_ids(mut self, value: Vec<i64>) -> Self {
        self.monitor_ids = Some(value);
        self
    }

    pub fn monitor_tags(mut self, value: Vec<String>) -> Self {
        self.monitor_tags = Some(value);
        self
    }

    pub fn query(mut self, value: crate::datadogV1::model::ServiceLevelObjectiveQuery) -> Self {
        self.query = Some(value);
        self
    }

    pub fn sli_specification(mut self, value: crate::datadogV1::model::SLOSliSpec) -> Self {
        self.sli_specification = Some(value);
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

impl<'de> Deserialize<'de> for ServiceLevelObjective {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceLevelObjectiveVisitor;
        impl<'a> Visitor<'a> for ServiceLevelObjectiveVisitor {
            type Value = ServiceLevelObjective;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<i64> = None;
                let mut creator: Option<crate::datadogV1::model::Creator> = None;
                let mut description: Option<Option<String>> = None;
                let mut groups: Option<Vec<String>> = None;
                let mut id: Option<String> = None;
                let mut modified_at: Option<i64> = None;
                let mut monitor_ids: Option<Vec<i64>> = None;
                let mut monitor_tags: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut query: Option<crate::datadogV1::model::ServiceLevelObjectiveQuery> = None;
                let mut sli_specification: Option<crate::datadogV1::model::SLOSliSpec> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut target_threshold: Option<f64> = None;
                let mut thresholds: Option<Vec<crate::datadogV1::model::SLOThreshold>> = None;
                let mut timeframe: Option<crate::datadogV1::model::SLOTimeframe> = None;
                let mut type_: Option<crate::datadogV1::model::SLOType> = None;
                let mut warning_threshold: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creator" => {
                            if v.is_null() {
                                continue;
                            }
                            creator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_ids" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            monitor_tags =
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
                        "sli_specification" => {
                            if v.is_null() {
                                continue;
                            }
                            sli_specification =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _sli_specification) = sli_specification {
                                match _sli_specification {
                                    crate::datadogV1::model::SLOSliSpec::UnparsedObject(
                                        _sli_specification,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
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

                let content = ServiceLevelObjective {
                    created_at,
                    creator,
                    description,
                    groups,
                    id,
                    modified_at,
                    monitor_ids,
                    monitor_tags,
                    name,
                    query,
                    sli_specification,
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

        deserializer.deserialize_any(ServiceLevelObjectiveVisitor)
    }
}
