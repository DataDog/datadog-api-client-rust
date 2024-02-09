// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request object to bulk configure tags for metrics matching the given prefix.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricBulkTagConfigCreate {
    /// Optional parameters for bulk creating metric tag configurations.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::MetricBulkTagConfigCreateAttributes>,
    /// A text prefix to match against metric names.
    #[serde(rename = "id")]
    pub id: String,
    /// The metric bulk configure tags resource.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::MetricBulkConfigureTagsType,
}

impl MetricBulkTagConfigCreate {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::MetricBulkConfigureTagsType,
    ) -> MetricBulkTagConfigCreate {
        MetricBulkTagConfigCreate {
            attributes: None,
            id,
            type_,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::MetricBulkTagConfigCreateAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }
}
