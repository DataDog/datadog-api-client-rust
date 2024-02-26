// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request object that includes the metric that you would like to configure tags for.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricTagConfigurationCreateRequest {
    /// Object for a single metric to be configure tags on.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::MetricTagConfigurationCreateData,
}

impl MetricTagConfigurationCreateRequest {
    pub fn new(
        data: crate::datadogV2::model::MetricTagConfigurationCreateData,
    ) -> MetricTagConfigurationCreateRequest {
        MetricTagConfigurationCreateRequest { data }
    }
}
