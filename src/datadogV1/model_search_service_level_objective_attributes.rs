// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchServiceLevelObjectiveAttributes {
    /// A list of tags associated with this service level objective.
Always included in service level objective responses (but may be empty).
    #[serde(rename = "all_tags", skip_serializing_if = "Option::is_none")]
    pub all_tags: Vec<String>,
    /// Creation timestamp (UNIX time in seconds)

Always included in service level objective responses.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: i64,
    /// The creator of the SLO
    #[serde(rename = "creator", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub creator: NullableSLOCreator,
    /// A user-defined description of the service level objective.

Always included in service level objective responses (but may be `null`).
Optional in create/update requests.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    /// Tags with the `env` tag key.
    #[serde(rename = "env_tags", skip_serializing_if = "Option::is_none")]
    pub env_tags: Vec<String>,
    /// A list of (up to 100) monitor groups that narrow the scope of a monitor service level objective.
Included in service level objective responses if it is not empty.
    #[serde(rename = "groups", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub groups: datadog.NullableList[String],
    /// Modification timestamp (UNIX time in seconds)

Always included in service level objective responses.
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: i64,
    /// A list of monitor ids that defines the scope of a monitor service level
objective.
    #[serde(rename = "monitor_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub monitor_ids: datadog.NullableList[i64],
    /// The name of the service level objective object.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// calculated status and error budget remaining.
    #[serde(rename = "overall_status", skip_serializing_if = "Option::is_none")]
    pub overall_status: Vec<SLOOverallStatuses>,
    /// A metric-based SLO. **Required if type is `metric`**. Note that Datadog only allows the sum by aggregator
to be used because this will sum up all request counts instead of averaging them, or taking the max or
min of all of those requests.
    #[serde(rename = "query", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub query: NullableSearchSLOQuery,
    /// Tags with the `service` tag key.
    #[serde(rename = "service_tags", skip_serializing_if = "Option::is_none")]
    pub service_tags: Vec<String>,
    /// The type of the service level objective.
    #[serde(rename = "slo_type", skip_serializing_if = "Option::is_none")]
    pub slo_type: SLOType,
    /// Status of the SLO's primary timeframe.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: SLOStatus,
    /// Tags with the `team` tag key.
    #[serde(rename = "team_tags", skip_serializing_if = "Option::is_none")]
    pub team_tags: Vec<String>,
    /// The thresholds (timeframes and associated targets) for this service level
objective object.
    #[serde(rename = "thresholds", skip_serializing_if = "Option::is_none")]
    pub thresholds: Vec<SearchSLOThreshold>,
}

