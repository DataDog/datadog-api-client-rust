// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupScalarColumn {
    /// The name of the tag key or group.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The type of column present.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: String,
    /// The array of tag values for each group found for the results of the formulas or queries.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Vec<Vec<String>>,
}

