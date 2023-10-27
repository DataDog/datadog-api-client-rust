// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpansMetricUpdateData {
    /// The span-based metric properties that will be updated.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::SpansMetricUpdateAttributes>,
    /// The type of resource. The value should always be spans_metrics.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SpansMetricType,
}

impl SpansMetricUpdateData {
    /// The new span-based metric properties.
    pub fn new(
        attributes: crate::datadogV2::model::SpansMetricUpdateAttributes,
        type_: crate::datadogV2::model::SpansMetricType,
    ) -> SpansMetricUpdateData {
        SpansMetricUpdateData {
            attributes: Box::new(attributes),
            type_,
        }
    }
}
