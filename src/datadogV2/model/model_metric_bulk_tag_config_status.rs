// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The status of a request to bulk configure metric tags.
/// It contains the fields from the original request for reference.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricBulkTagConfigStatus {
    /// Optional attributes for the status of a bulk tag configuration request.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::MetricBulkTagConfigStatusAttributes>,
    /// A text prefix to match against metric names.
    #[serde(rename = "id")]
    pub id: String,
    /// The metric bulk configure tags resource.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::MetricBulkConfigureTagsType,
}

impl MetricBulkTagConfigStatus {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::MetricBulkConfigureTagsType,
    ) -> MetricBulkTagConfigStatus {
        MetricBulkTagConfigStatus {
            attributes: None,
            id,
            type_,
        }
    }

    pub fn with_attributes(
        &mut self,
        value: crate::datadogV2::model::MetricBulkTagConfigStatusAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }
}
