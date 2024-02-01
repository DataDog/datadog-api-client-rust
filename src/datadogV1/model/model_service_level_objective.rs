// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A service level objective object includes a service level indicator, thresholds
/// for one or more timeframes, and metadata (`name`, `description`, `tags`, etc.).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
            tags: None,
            target_threshold: None,
            thresholds,
            timeframe: None,
            type_,
            warning_threshold: None,
        }
    }

    pub fn created_at(&mut self, value: i64) -> &mut Self {
        self.created_at = Some(value);
        self
    }

    pub fn creator(&mut self, value: crate::datadogV1::model::Creator) -> &mut Self {
        self.creator = Some(value);
        self
    }

    pub fn description(&mut self, value: Option<String>) -> &mut Self {
        self.description = Some(value);
        self
    }

    pub fn groups(&mut self, value: Vec<String>) -> &mut Self {
        self.groups = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn modified_at(&mut self, value: i64) -> &mut Self {
        self.modified_at = Some(value);
        self
    }

    pub fn monitor_ids(&mut self, value: Vec<i64>) -> &mut Self {
        self.monitor_ids = Some(value);
        self
    }

    pub fn monitor_tags(&mut self, value: Vec<String>) -> &mut Self {
        self.monitor_tags = Some(value);
        self
    }

    pub fn query(
        &mut self,
        value: crate::datadogV1::model::ServiceLevelObjectiveQuery,
    ) -> &mut Self {
        self.query = Some(value);
        self
    }

    pub fn tags(&mut self, value: Vec<String>) -> &mut Self {
        self.tags = Some(value);
        self
    }

    pub fn target_threshold(&mut self, value: f64) -> &mut Self {
        self.target_threshold = Some(value);
        self
    }

    pub fn timeframe(&mut self, value: crate::datadogV1::model::SLOTimeframe) -> &mut Self {
        self.timeframe = Some(value);
        self
    }

    pub fn warning_threshold(&mut self, value: f64) -> &mut Self {
        self.warning_threshold = Some(value);
        self
    }
}
