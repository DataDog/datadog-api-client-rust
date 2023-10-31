// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The new span-based metric properties.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpansMetricCreateData {
    /// The object describing the Datadog span-based metric to create.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::SpansMetricCreateAttributes>,
    /// The name of the span-based metric.
    #[serde(rename = "id")]
    pub id: String,
    /// The type of resource. The value should always be spans_metrics.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SpansMetricType,
}

impl SpansMetricCreateData {
    pub fn new(
        attributes: crate::datadogV2::model::SpansMetricCreateAttributes,
        id: String,
        type_: crate::datadogV2::model::SpansMetricType,
    ) -> SpansMetricCreateData {
        SpansMetricCreateData {
            attributes: Box::new(attributes),
            id,
            type_,
        }
    }
}
