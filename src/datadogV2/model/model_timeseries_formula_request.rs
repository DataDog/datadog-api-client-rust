// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A single timeseries query to be executed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeseriesFormulaRequest {
    /// The object describing a timeseries formula request.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::TimeseriesFormulaRequestAttributes,
    /// The type of the resource. The value should always be timeseries_request.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::TimeseriesFormulaRequestType,
}

impl TimeseriesFormulaRequest {
    pub fn new(
        attributes: crate::datadogV2::model::TimeseriesFormulaRequestAttributes,
        type_: crate::datadogV2::model::TimeseriesFormulaRequestType,
    ) -> TimeseriesFormulaRequest {
        TimeseriesFormulaRequest { attributes, type_ }
    }
}
