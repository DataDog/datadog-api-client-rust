// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageTimeSeriesObject {
    /// Datetime in ISO-8601 format, UTC. The hour for the usage.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Contains the number measured for the given usage_type during the hour.
    #[serde(
        rename = "value",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<Option<i64>>,
}

impl UsageTimeSeriesObject {
    /// Usage timeseries data.
    pub fn new() -> UsageTimeSeriesObject {
        UsageTimeSeriesObject {
            timestamp: None,
            value: None,
        }
    }
}
