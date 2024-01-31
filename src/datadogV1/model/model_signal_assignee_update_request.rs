// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes describing an assignee update operation over a security signal.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignalAssigneeUpdateRequest {
    /// The UUID of the user being assigned. Use empty string to return signal to unassigned.
    #[serde(rename = "assignee")]
    pub assignee: String,
    /// Version of the updated signal. If server side version is higher, update will be rejected.
    #[serde(rename = "version")]
    pub version: Option<i64>,
}

impl SignalAssigneeUpdateRequest {
    pub fn new(assignee: String) -> SignalAssigneeUpdateRequest {
        SignalAssigneeUpdateRequest {
            assignee,
            version: None,
        }
    }

    pub fn with_version(&mut self, value: i64) -> &mut Self {
        self.version = Some(value);
        self
    }
}
