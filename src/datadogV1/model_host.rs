// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Host {
    /// Host aliases collected by Datadog.
    #[serde(rename = "aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Vec<String>,
    /// The Datadog integrations reporting metrics for the host.
    #[serde(rename = "apps", skip_serializing_if = "Option::is_none")]
    pub apps: Vec<String>,
    /// AWS name of your host.
    #[serde(rename = "aws_name", skip_serializing_if = "Option::is_none")]
    pub aws_name: String,
    /// The host name.
    #[serde(rename = "host_name", skip_serializing_if = "Option::is_none")]
    pub host_name: String,
    /// The host ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: i64,
    /// If a host is muted or unmuted.
    #[serde(rename = "is_muted", skip_serializing_if = "Option::is_none")]
    pub is_muted: bool,
    /// Last time the host reported a metric data point.
    #[serde(rename = "last_reported_time", skip_serializing_if = "Option::is_none")]
    pub last_reported_time: i64,
    /// Metadata associated with your host.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: HostMeta,
    /// Host Metrics collected.
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: HostMetrics,
    /// Timeout of the mute applied to your host.
    #[serde(rename = "mute_timeout", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub mute_timeout: Option<Int64>,
    /// The host name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Source or cloud provider associated with your host.
    #[serde(rename = "sources", skip_serializing_if = "Option::is_none")]
    pub sources: Vec<String>,
    /// List of tags for each source (AWS, Datadog Agent, Chef..).
    #[serde(rename = "tags_by_source", skip_serializing_if = "Option::is_none")]
    pub tags_by_source: map[string]Vec<String>,
    /// Displays UP when the expected metrics are received and displays `???` if no metrics are received.
    #[serde(rename = "up", skip_serializing_if = "Option::is_none")]
    pub up: bool,
}

