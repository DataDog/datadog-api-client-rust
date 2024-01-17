// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A column containing the tag keys and values in a group.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupScalarColumn {
    /// The name of the tag key or group.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The type of column present for groups.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ScalarColumnTypeGroup>,
    /// The array of tag values for each group found for the results of the formulas or queries.
    #[serde(rename = "values")]
    pub values: Option<Vec<Vec<String>>>,
}

impl GroupScalarColumn {
    pub fn new() -> GroupScalarColumn {
        GroupScalarColumn {
            name: None,
            type_: None,
            values: None,
        }
    }
}
