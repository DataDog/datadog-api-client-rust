// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageLogsHour {
    /// Contains the number of billable log bytes ingested.
    #[serde(rename = "billable_ingested_bytes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub billable_ingested_bytes: Option<Int64>,
    /// The hour for the usage.
    #[serde(rename = "hour", skip_serializing_if = "Option::is_none")]
    pub hour: String,
    /// Contains the number of log events indexed.
    #[serde(rename = "indexed_events_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub indexed_events_count: Option<Int64>,
    /// Contains the number of log bytes ingested.
    #[serde(rename = "ingested_events_bytes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub ingested_events_bytes: Option<Int64>,
    /// Contains the number of logs forwarded bytes (data available as of April 1st 2023)
    #[serde(rename = "logs_forwarding_events_bytes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub logs_forwarding_events_bytes: Option<Int64>,
    /// Contains the number of live log events indexed (data available as of December 1, 2020).
    #[serde(rename = "logs_live_indexed_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub logs_live_indexed_count: Option<Int64>,
    /// Contains the number of live log bytes ingested (data available as of December 1, 2020).
    #[serde(rename = "logs_live_ingested_bytes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub logs_live_ingested_bytes: Option<Int64>,
    /// Contains the number of rehydrated log events indexed (data available as of December 1, 2020).
    #[serde(rename = "logs_rehydrated_indexed_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub logs_rehydrated_indexed_count: Option<Int64>,
    /// Contains the number of rehydrated log bytes ingested (data available as of December 1, 2020).
    #[serde(rename = "logs_rehydrated_ingested_bytes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub logs_rehydrated_ingested_bytes: Option<Int64>,
    /// The organization name.
    #[serde(rename = "org_name", skip_serializing_if = "Option::is_none")]
    pub org_name: String,
    /// The organization public ID.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
}

