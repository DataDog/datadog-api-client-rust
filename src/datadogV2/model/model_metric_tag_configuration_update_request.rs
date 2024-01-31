// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request object that includes the metric that you would like to edit the tag configuration on.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricTagConfigurationUpdateRequest {
    /// Object for a single tag configuration to be edited.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::MetricTagConfigurationUpdateData,
}

impl MetricTagConfigurationUpdateRequest {
    pub fn new(
        data: crate::datadogV2::model::MetricTagConfigurationUpdateData,
    ) -> MetricTagConfigurationUpdateRequest {
        MetricTagConfigurationUpdateRequest { data }
    }
}
