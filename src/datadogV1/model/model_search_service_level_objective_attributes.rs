// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A service level objective object includes a service level indicator, thresholds
/// for one or more timeframes, and metadata (`name`, `description`, and `tags`).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
        }
    }

    pub fn all_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.all_tags = Some(value);
        self
    }

    pub fn created_at(&mut self, value: i64) -> &mut Self {
        self.created_at = Some(value);
        self
    }

    pub fn creator(&mut self, value: Option<crate::datadogV1::model::SLOCreator>) -> &mut Self {
        self.creator = Some(value);
        self
    }

    pub fn description(&mut self, value: Option<String>) -> &mut Self {
        self.description = Some(value);
        self
    }

    pub fn env_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.env_tags = Some(value);
        self
    }

    pub fn groups(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.groups = Some(value);
        self
    }

    pub fn modified_at(&mut self, value: i64) -> &mut Self {
        self.modified_at = Some(value);
        self
    }

    pub fn monitor_ids(&mut self, value: Option<Vec<i64>>) -> &mut Self {
        self.monitor_ids = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn overall_status(
        &mut self,
        value: Vec<crate::datadogV1::model::SLOOverallStatuses>,
    ) -> &mut Self {
        self.overall_status = Some(value);
        self
    }

    pub fn query(&mut self, value: Option<crate::datadogV1::model::SearchSLOQuery>) -> &mut Self {
        self.query = Some(value);
        self
    }

    pub fn service_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.service_tags = Some(value);
        self
    }

    pub fn slo_type(&mut self, value: crate::datadogV1::model::SLOType) -> &mut Self {
        self.slo_type = Some(value);
        self
    }

    pub fn status(&mut self, value: crate::datadogV1::model::SLOStatus) -> &mut Self {
        self.status = Some(value);
        self
    }

    pub fn team_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.team_tags = Some(value);
        self
    }

    pub fn thresholds(
        &mut self,
        value: Vec<crate::datadogV1::model::SearchSLOThreshold>,
    ) -> &mut Self {
        self.thresholds = Some(value);
        self
    }
}

impl Default for SearchServiceLevelObjectiveAttributes {
    fn default() -> Self {
        Self::new()
    }
}
