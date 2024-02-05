// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Updated funnel widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunnelQuery {
    /// Source from which to query items to display in the funnel.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::FunnelSource,
    /// The widget query.
    #[serde(rename = "query_string")]
    pub query_string: String,
    /// List of funnel steps.
    #[serde(rename = "steps")]
    pub steps: Vec<crate::datadogV1::model::FunnelStep>,
}

impl FunnelQuery {
    pub fn new(
        data_source: crate::datadogV1::model::FunnelSource,
        query_string: String,
        steps: Vec<crate::datadogV1::model::FunnelStep>,
    ) -> FunnelQuery {
        FunnelQuery {
            data_source,
            query_string,
            steps,
        }
    }
}
