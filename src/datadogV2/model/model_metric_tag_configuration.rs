// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object for a single metric tag configuration.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricTagConfiguration {
    /// Object containing the definition of a metric tag configuration attributes.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::MetricTagConfigurationAttributes>>,
    /// The metric name for this resource.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The metric tag configuration resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::MetricTagConfigurationType>,
}

impl MetricTagConfiguration {
    pub fn new() -> MetricTagConfiguration {
        MetricTagConfiguration {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
