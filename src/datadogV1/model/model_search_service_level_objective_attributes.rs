// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A service level objective object includes a service level indicator, thresholds
/// for one or more timeframes, and metadata (`name`, `description`, and `tags`).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SearchServiceLevelObjectiveAttributes {
    /// A list of tags associated with this service level objective.
    /// Always included in service level objective responses (but may be empty).
    #[serde(rename = "all_tags")]
    pub all_tags: Option<Vec<String>>,
    /// Creation timestamp (UNIX time in seconds)
    ///
    /// Always included in service level objective responses.
    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,
    /// The creator of the SLO
    #[serde(
        rename = "creator",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub creator: Option<Option<crate::datadogV1::model::SLOCreator>>,
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
    /// Tags with the `env` tag key.
    #[serde(rename = "env_tags")]
    pub env_tags: Option<Vec<String>>,
    /// A list of (up to 100) monitor groups that narrow the scope of a monitor service level objective.
    /// Included in service level objective responses if it is not empty.
    #[serde(rename = "groups", default, with = "::serde_with::rust::double_option")]
    pub groups: Option<Option<Vec<String>>>,
    /// Modification timestamp (UNIX time in seconds)
    ///
    /// Always included in service level objective responses.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<i64>,
    /// A list of monitor ids that defines the scope of a monitor service level
    /// objective.
    #[serde(
        rename = "monitor_ids",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub monitor_ids: Option<Option<Vec<i64>>>,
    /// The name of the service level objective object.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// calculated status and error budget remaining.
    #[serde(rename = "overall_status")]
    pub overall_status: Option<Vec<crate::datadogV1::model::SLOOverallStatuses>>,
    /// A metric-based SLO. **Required if type is `metric`**. Note that Datadog only allows the sum by aggregator
    /// to be used because this will sum up all request counts instead of averaging them, or taking the max or
    /// min of all of those requests.
    #[serde(rename = "query", default, with = "::serde_with::rust::double_option")]
    pub query: Option<Option<crate::datadogV1::model::SearchSLOQuery>>,
    /// Tags with the `service` tag key.
    #[serde(rename = "service_tags")]
    pub service_tags: Option<Vec<String>>,
    /// The type of the service level objective.
    #[serde(rename = "slo_type")]
    pub slo_type: Option<crate::datadogV1::model::SLOType>,
    /// Status of the SLO's primary timeframe.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::SLOStatus>,
    /// Tags with the `team` tag key.
    #[serde(rename = "team_tags")]
    pub team_tags: Option<Vec<String>>,
    /// The thresholds (timeframes and associated targets) for this service level
    /// objective object.
    #[serde(rename = "thresholds")]
    pub thresholds: Option<Vec<crate::datadogV1::model::SearchSLOThreshold>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SearchServiceLevelObjectiveAttributes {
    pub fn new() -> SearchServiceLevelObjectiveAttributes {
        SearchServiceLevelObjectiveAttributes {
            all_tags: None,
            created_at: None,
            creator: None,
            description: None,
            env_tags: None,
            groups: None,
            modified_at: None,
            monitor_ids: None,
            name: None,
            overall_status: None,
            query: None,
            service_tags: None,
            slo_type: None,
            status: None,
            team_tags: None,
            thresholds: None,
            _unparsed: false,
        }
    }

    pub fn all_tags(mut self, value: Vec<String>) -> Self {
        self.all_tags = Some(value);
        self
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn creator(mut self, value: Option<crate::datadogV1::model::SLOCreator>) -> Self {
        self.creator = Some(value);
        self
    }

    pub fn description(mut self, value: Option<String>) -> Self {
        self.description = Some(value);
        self
    }

    pub fn env_tags(mut self, value: Vec<String>) -> Self {
        self.env_tags = Some(value);
        self
    }

    pub fn groups(mut self, value: Option<Vec<String>>) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn modified_at(mut self, value: i64) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn monitor_ids(mut self, value: Option<Vec<i64>>) -> Self {
        self.monitor_ids = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn overall_status(
        mut self,
        value: Vec<crate::datadogV1::model::SLOOverallStatuses>,
    ) -> Self {
        self.overall_status = Some(value);
        self
    }

    pub fn query(mut self, value: Option<crate::datadogV1::model::SearchSLOQuery>) -> Self {
        self.query = Some(value);
        self
    }

    pub fn service_tags(mut self, value: Vec<String>) -> Self {
        self.service_tags = Some(value);
        self
    }

    pub fn slo_type(mut self, value: crate::datadogV1::model::SLOType) -> Self {
        self.slo_type = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV1::model::SLOStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn team_tags(mut self, value: Vec<String>) -> Self {
        self.team_tags = Some(value);
        self
    }

    pub fn thresholds(mut self, value: Vec<crate::datadogV1::model::SearchSLOThreshold>) -> Self {
        self.thresholds = Some(value);
        self
    }
}

impl Default for SearchServiceLevelObjectiveAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SearchServiceLevelObjectiveAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SearchServiceLevelObjectiveAttributesVisitor;
        impl<'a> Visitor<'a> for SearchServiceLevelObjectiveAttributesVisitor {
            type Value = SearchServiceLevelObjectiveAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut all_tags: Option<Vec<String>> = None;
                let mut created_at: Option<i64> = None;
                let mut creator: Option<Option<crate::datadogV1::model::SLOCreator>> = None;
                let mut description: Option<Option<String>> = None;
                let mut env_tags: Option<Vec<String>> = None;
                let mut groups: Option<Option<Vec<String>>> = None;
                let mut modified_at: Option<i64> = None;
                let mut monitor_ids: Option<Option<Vec<i64>>> = None;
                let mut name: Option<String> = None;
                let mut overall_status: Option<Vec<crate::datadogV1::model::SLOOverallStatuses>> =
                    None;
                let mut query: Option<Option<crate::datadogV1::model::SearchSLOQuery>> = None;
                let mut service_tags: Option<Vec<String>> = None;
                let mut slo_type: Option<crate::datadogV1::model::SLOType> = None;
                let mut status: Option<crate::datadogV1::model::SLOStatus> = None;
                let mut team_tags: Option<Vec<String>> = None;
                let mut thresholds: Option<Vec<crate::datadogV1::model::SearchSLOThreshold>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "all_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            all_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creator" => {
                            creator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            env_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "groups" => {
                            groups = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitor_ids" => {
                            monitor_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "overall_status" => {
                            if v.is_null() {
                                continue;
                            }
                            overall_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            service_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slo_type" => {
                            if v.is_null() {
                                continue;
                            }
                            slo_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _slo_type) = slo_type {
                                match _slo_type {
                                    crate::datadogV1::model::SLOType::UnparsedObject(_slo_type) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "team_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            team_tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "thresholds" => {
                            if v.is_null() {
                                continue;
                            }
                            thresholds = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SearchServiceLevelObjectiveAttributes {
                    all_tags,
                    created_at,
                    creator,
                    description,
                    env_tags,
                    groups,
                    modified_at,
                    monitor_ids,
                    name,
                    overall_status,
                    query,
                    service_tags,
                    slo_type,
                    status,
                    team_tags,
                    thresholds,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SearchServiceLevelObjectiveAttributesVisitor)
    }
}
