// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// An object containing service check and status.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCheck {
    /// The check.
    #[serde(rename = "check")]
    pub check: String,
    /// The host name correlated with the check.
    #[serde(rename = "host_name")]
    pub host_name: String,
    /// Message containing check status.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The status of a service check. Set to `0` for OK, `1` for warning, `2` for critical, and `3` for unknown.
    #[serde(rename = "status")]
    pub status: crate::datadogV1::model::ServiceCheckStatus,
    /// Tags related to a check.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// Time of check.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
}

impl ServiceCheck {
    pub fn new(
        check: String,
        host_name: String,
        status: crate::datadogV1::model::ServiceCheckStatus,
        tags: Vec<String>,
    ) -> ServiceCheck {
        ServiceCheck {
            check,
            host_name,
            message: None,
            status,
            tags,
            timestamp: None,
        }
    }

    pub fn message(&mut self, value: String) -> &mut Self {
        self.message = Some(value);
        self
    }

    pub fn timestamp(&mut self, value: i64) -> &mut Self {
        self.timestamp = Some(value);
        self
    }
}
