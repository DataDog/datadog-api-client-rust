// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A column containing the numerical results for a formula or query.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataScalarColumn {
    /// Metadata for the resulting numerical values.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV2::model::ScalarMeta>>,
    /// The name referencing the formula or query for this column.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The type of column present for numbers.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ScalarColumnTypeNumber>,
    /// The array of numerical values for one formula or query.
    #[serde(rename = "values")]
    pub values: Option<Vec<Option<f64>>>,
}

impl DataScalarColumn {
    pub fn new() -> DataScalarColumn {
        DataScalarColumn {
            meta: None,
            name: None,
            type_: None,
            values: None,
        }
    }
}