// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListStreamQuery {
    /// Compute configuration for the List Stream Widget. Compute can be used only with the logs_transaction_stream (from 1 to 5 items) list stream source.
    #[serde(rename = "compute", skip_serializing_if = "Option::is_none")]
    pub compute: Vec<ListStreamComputeItems>,
    /// Source from which to query items to display in the stream.
    #[serde(rename = "data_source", skip_serializing_if = "Option::is_none")]
    pub data_source: ListStreamSource,
    /// Size to use to display an event.
    #[serde(rename = "event_size", skip_serializing_if = "Option::is_none")]
    pub event_size: WidgetEventSize,
    /// Group by configuration for the List Stream Widget. Group by can be used only with logs_pattern_stream (up to 3 items) or logs_transaction_stream (one group by item is required) list stream source.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Vec<ListStreamGroupByItems>,
    /// List of indexes.
    #[serde(rename = "indexes", skip_serializing_if = "Option::is_none")]
    pub indexes: Vec<String>,
    /// Widget query.
    #[serde(rename = "query_string", skip_serializing_if = "Option::is_none")]
    pub query_string: String,
    /// Which column and order to sort by
    #[serde(rename = "sort")]
    pub sort: WidgetFieldSort,
    /// Option for storage location. Feature in Private Beta.
    #[serde(rename = "storage", skip_serializing_if = "Option::is_none")]
    pub storage: String,
}

