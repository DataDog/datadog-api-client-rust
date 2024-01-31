// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response of a monitor group search.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorGroupSearchResponse {
    /// The counts of monitor groups per different criteria.
    #[serde(rename = "counts")]
    pub counts: Option<crate::datadogV1::model::MonitorGroupSearchResponseCounts>,
    /// The list of found monitor groups.
    #[serde(rename = "groups")]
    pub groups: Option<Vec<crate::datadogV1::model::MonitorGroupSearchResult>>,
    /// Metadata about the response.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::MonitorSearchResponseMetadata>,
}

impl MonitorGroupSearchResponse {
    pub fn new() -> MonitorGroupSearchResponse {
        MonitorGroupSearchResponse {
            counts: None,
            groups: None,
            metadata: None,
        }
    }

    pub fn with_counts(
        &mut self,
        value: crate::datadogV1::model::MonitorGroupSearchResponseCounts,
    ) -> &mut Self {
        self.counts = Some(value);
        self
    }

    pub fn with_groups(
        &mut self,
        value: Vec<crate::datadogV1::model::MonitorGroupSearchResult>,
    ) -> &mut Self {
        self.groups = Some(value);
        self
    }

    pub fn with_metadata(
        &mut self,
        value: crate::datadogV1::model::MonitorSearchResponseMetadata,
    ) -> &mut Self {
        self.metadata = Some(value);
        self
    }
}
impl Default for MonitorGroupSearchResponse {
    fn default() -> Self {
        Self::new()
    }
}
