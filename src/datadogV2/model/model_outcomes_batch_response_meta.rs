// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Metadata pertaining to the bulk operation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutcomesBatchResponseMeta {
    /// Total number of scorecard results received during the bulk operation.
    #[serde(rename = "total_received")]
    pub total_received: Option<i64>,
    /// Total number of scorecard results modified during the bulk operation.
    #[serde(rename = "total_updated")]
    pub total_updated: Option<i64>,
}

impl OutcomesBatchResponseMeta {
    pub fn new() -> OutcomesBatchResponseMeta {
        OutcomesBatchResponseMeta {
            total_received: None,
            total_updated: None,
        }
    }

    pub fn total_received(&mut self, value: i64) -> &mut Self {
        self.total_received = Some(value);
        self
    }

    pub fn total_updated(&mut self, value: i64) -> &mut Self {
        self.total_updated = Some(value);
        self
    }
}

impl Default for OutcomesBatchResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}
