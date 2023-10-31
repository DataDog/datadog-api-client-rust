// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Timeframe to retrieve the log from.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LogsListRequestTime {
    /// Minimum timestamp for requested logs.
    #[serde(rename = "from")]
    pub from: String,
    /// Timezone can be specified both as an offset (for example "UTC+03:00")
    /// or a regional zone (for example "Europe/Paris").
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    /// Maximum timestamp for requested logs.
    #[serde(rename = "to")]
    pub to: String,
}

impl LogsListRequestTime {
    pub fn new(from: String, to: String) -> LogsListRequestTime {
        LogsListRequestTime {
            from,
            timezone: None,
            to,
        }
    }
}
