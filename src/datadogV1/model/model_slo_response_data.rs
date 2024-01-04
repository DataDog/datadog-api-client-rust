// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A service level objective object includes a service level indicator, thresholds
/// for one or more timeframes, and metadata (`name`, `description`, `tags`, etc.).
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SLOResponseData {
    /// A list of SLO monitors IDs that reference this SLO. This field is returned only when `with_configured_alert_ids` parameter is true in query.
    #[serde(rename = "configured_alert_ids")]
    pub configured_alert_ids: Option<Vec<i64>>,
    /// Creation timestamp (UNIX time in seconds)
    ///
    /// Always included in service level objective responses.
    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,
    /// Object describing the creator of the shared element.
    #[serde(rename = "creator")]
    pub creator: Option<Box<crate::datadogV1::model::Creator>>,
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
    /// A list of (up to 20) monitor groups that narrow the scope of a monitor service level objective.
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
    /// Always included in service level objective responses for monitor service level
    /// objectives (but may be empty). Ignored in create/update requests. Does not
    /// affect which monitors are included in the service level objective (that is
    /// determined entirely by the `monitor_ids` field).
    #[serde(rename = "monitor_tags")]
    pub monitor_tags: Option<Vec<String>>,
    /// The name of the service level objective object.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// A metric-based SLO. **Required if type is `metric`**. Note that Datadog only allows the sum by aggregator
    /// to be used because this will sum up all request counts instead of averaging them, or taking the max or
    /// min of all of those requests.
    #[serde(rename = "query")]
    pub query: Option<Box<crate::datadogV1::model::ServiceLevelObjectiveQuery>>,
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
    pub thresholds: Option<Vec<crate::datadogV1::model::SLOThreshold>>,
    /// The SLO time window options.
    #[serde(rename = "timeframe")]
    pub timeframe: Option<crate::datadogV1::model::SLOTimeframe>,
    /// The type of the service level objective.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::SLOType>,
    /// The optional warning threshold such that when the service level indicator is
    /// below this value for the given threshold, but above the target threshold, the
    /// objective appears in a "warning" state. This value must be greater than the target
    /// threshold.
    #[serde(rename = "warning_threshold")]
    pub warning_threshold: Option<f64>,
}

impl SLOResponseData {
    pub fn new() -> SLOResponseData {
        SLOResponseData {
            configured_alert_ids: None,
            created_at: None,
            creator: None,
            description: None,
            groups: None,
            id: None,
            modified_at: None,
            monitor_ids: None,
            monitor_tags: None,
            name: None,
            query: None,
            tags: None,
            target_threshold: None,
            thresholds: None,
            timeframe: None,
            type_: None,
            warning_threshold: None,
        }
    }
}
