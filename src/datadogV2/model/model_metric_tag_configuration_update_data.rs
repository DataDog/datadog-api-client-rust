// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object for a single tag configuration to be edited.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricTagConfigurationUpdateData {
    /// Object containing the definition of a metric tag configuration to be updated.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::MetricTagConfigurationUpdateAttributes>,
    /// The metric name for this resource.
    #[serde(rename = "id")]
    pub id: String,
    /// The metric tag configuration resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::MetricTagConfigurationType,
}

impl MetricTagConfigurationUpdateData {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::MetricTagConfigurationType,
    ) -> MetricTagConfigurationUpdateData {
        MetricTagConfigurationUpdateData {
            attributes: None,
            id,
            type_,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::MetricTagConfigurationUpdateAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }
}
