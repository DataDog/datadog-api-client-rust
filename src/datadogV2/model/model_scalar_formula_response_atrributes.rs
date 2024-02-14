// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object describing a scalar response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalarFormulaResponseAtrributes {
    /// List of response columns, each corresponding to an individual formula or query in the request and with values in parallel arrays matching the series list.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<crate::datadogV2::model::ScalarColumn>>,
}

impl ScalarFormulaResponseAtrributes {
    pub fn new() -> ScalarFormulaResponseAtrributes {
        ScalarFormulaResponseAtrributes { columns: None }
    }

    pub fn columns(&mut self, value: Vec<crate::datadogV2::model::ScalarColumn>) -> &mut Self {
        self.columns = Some(value);
        self
    }
}

impl Default for ScalarFormulaResponseAtrributes {
    fn default() -> Self {
        Self::new()
    }
}
