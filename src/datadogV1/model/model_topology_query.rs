// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Query to service-based topology data sources like the service map or data streams.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopologyQuery {
    /// Name of the data source
    #[serde(rename = "data_source")]
    pub data_source: Option<crate::datadogV1::model::TopologyQueryDataSource>,
    /// Your environment and primary tag (or * if enabled for your account).
    #[serde(rename = "filters")]
    pub filters: Option<Vec<String>>,
    /// Name of the service
    #[serde(rename = "service")]
    pub service: Option<String>,
}

impl TopologyQuery {
    pub fn new() -> TopologyQuery {
        TopologyQuery {
            data_source: None,
            filters: None,
            service: None,
        }
    }

    pub fn data_source(mut self, value: crate::datadogV1::model::TopologyQueryDataSource) -> Self {
        self.data_source = Some(value);
        self
    }

    pub fn filters(mut self, value: Vec<String>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }
}

impl Default for TopologyQuery {
    fn default() -> Self {
        Self::new()
    }
}
