// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSDSHour {
    /// The total number of bytes scanned of APM usage across all usage types by the Sensitive Data Scanner from the start of the given hour’s month until the given hour.
    #[serde(rename = "apm_scanned_bytes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub apm_scanned_bytes: Option<Int64>,
    /// The total number of bytes scanned of Events usage across all usage types by the Sensitive Data Scanner from the start of the given hour’s month until the given hour.
    #[serde(rename = "events_scanned_bytes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub events_scanned_bytes: Option<Int64>,
    /// The hour for the usage.
    #[serde(rename = "hour", skip_serializing_if = "Option::is_none")]
    pub hour: String,
    /// The total number of bytes scanned of logs usage by the Sensitive Data Scanner from the start of the given hour’s month until the given hour.
    #[serde(rename = "logs_scanned_bytes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub logs_scanned_bytes: Option<Int64>,
    /// The organization name.
    #[serde(rename = "org_name", skip_serializing_if = "Option::is_none")]
    pub org_name: String,
    /// The organization public ID.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
    /// The total number of bytes scanned of RUM usage across all usage types by the Sensitive Data Scanner from the start of the given hour’s month until the given hour.
    #[serde(rename = "rum_scanned_bytes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub rum_scanned_bytes: Option<Int64>,
    /// The total number of bytes scanned across all usage types by the Sensitive Data Scanner from the start of the given hour’s month until the given hour.
    #[serde(rename = "total_scanned_bytes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub total_scanned_bytes: Option<Int64>,
}

