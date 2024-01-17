// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object for a single metric to be configure tags on.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricTagConfigurationCreateData {
    /// Object containing the definition of a metric tag configuration to be created.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::MetricTagConfigurationCreateAttributes>>,
    /// The metric name for this resource.
    #[serde(rename = "id")]
    pub id: String,
    /// The metric tag configuration resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::MetricTagConfigurationType,
}

impl MetricTagConfigurationCreateData {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::MetricTagConfigurationType,
    ) -> MetricTagConfigurationCreateData {
        MetricTagConfigurationCreateData {
            attributes: None,
            id,
            type_,
        }
    }
}
