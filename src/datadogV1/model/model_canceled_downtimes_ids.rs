// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing array of IDs of canceled downtimes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CanceledDowntimesIds {
    /// ID of downtimes that were canceled.
    #[serde(rename = "cancelled_ids")]
    pub cancelled_ids: Option<Vec<i64>>,
}

impl CanceledDowntimesIds {
    pub fn new() -> CanceledDowntimesIds {
        CanceledDowntimesIds {
            cancelled_ids: None,
        }
    }

    pub fn cancelled_ids(mut self, value: Vec<i64>) -> Self {
        self.cancelled_ids = Some(value);
        self
    }
}

impl Default for CanceledDowntimesIds {
    fn default() -> Self {
        Self::new()
    }
}
