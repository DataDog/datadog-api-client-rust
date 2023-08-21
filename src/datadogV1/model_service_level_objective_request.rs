// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceLevelObjectiveRequest {
    /// A user-defined description of the service level objective.

Always included in service level objective responses (but may be `null`).
Optional in create/update requests.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    /// A list of (up to 100) monitor groups that narrow the scope of a monitor service level objective.

Included in service level objective responses if it is not empty. Optional in
create/update requests for monitor service level objectives, but may only be
used when then length of the `monitor_ids` field is one.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Vec<String>,
    /// A list of monitor IDs that defines the scope of a monitor service level
objective. **Required if type is `monitor`**.
    #[serde(rename = "monitor_ids", skip_serializing_if = "Option::is_none")]
    pub monitor_ids: Vec<i64>,
    /// The name of the service level objective object.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// A metric-based SLO. **Required if type is `metric`**. Note that Datadog only allows the sum by aggregator
to be used because this will sum up all request counts instead of averaging them, or taking the max or
min of all of those requests.
    #[serde(rename = "query")]
    pub query: ServiceLevelObjectiveQuery,
    /// A list of tags associated with this service level objective.
Always included in service level objective responses (but may be empty).
Optional in create/update requests.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Vec<String>,
    /// The target threshold such that when the service level indicator is above this
threshold over the given timeframe, the objective is being met.
    #[serde(rename = "target_threshold", skip_serializing_if = "Option::is_none")]
    pub target_threshold: f64,
    /// The thresholds (timeframes and associated targets) for this service level
objective object.
    #[serde(rename = "thresholds", skip_serializing_if = "Option::is_none")]
    pub thresholds: Vec<SLOThreshold>,
    /// The SLO time window options.
    #[serde(rename = "timeframe", skip_serializing_if = "Option::is_none")]
    pub timeframe: SLOTimeframe,
    /// The type of the service level objective.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SLOType,
    /// The optional warning threshold such that when the service level indicator is
below this value for the given threshold, but above the target threshold, the
objective appears in a "warning" state. This value must be greater than the target
threshold.
    #[serde(rename = "warning_threshold", skip_serializing_if = "Option::is_none")]
    pub warning_threshold: f64,
}

