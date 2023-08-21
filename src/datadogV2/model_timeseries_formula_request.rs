// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeseriesFormulaRequest {
    /// The object describing a timeseries formula request.
    #[serde(rename = "attributes")]
    pub attributes: TimeseriesFormulaRequestAttributes,
    /// The type of the resource. The value should always be timeseries_request.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: TimeseriesFormulaRequestType,
}

