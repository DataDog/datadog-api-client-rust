// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Widget definition.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScatterPlotWidgetDefinitionRequests {
    /// Scatterplot request containing formulas and functions.
    #[serde(rename = "table")]
    pub table: Option<crate::datadogV1::model::ScatterplotTableRequest>,
    /// Updated scatter plot.
    #[serde(rename = "x")]
    pub x: Option<crate::datadogV1::model::ScatterPlotRequest>,
    /// Updated scatter plot.
    #[serde(rename = "y")]
    pub y: Option<crate::datadogV1::model::ScatterPlotRequest>,
}

impl ScatterPlotWidgetDefinitionRequests {
    pub fn new() -> ScatterPlotWidgetDefinitionRequests {
        ScatterPlotWidgetDefinitionRequests {
            table: None,
            x: None,
            y: None,
        }
    }

    pub fn with_table(
        &mut self,
        value: crate::datadogV1::model::ScatterplotTableRequest,
    ) -> &mut Self {
        self.table = Some(value);
        self
    }

    pub fn with_x(&mut self, value: crate::datadogV1::model::ScatterPlotRequest) -> &mut Self {
        self.x = Some(value);
        self
    }

    pub fn with_y(&mut self, value: crate::datadogV1::model::ScatterPlotRequest) -> &mut Self {
        self.y = Some(value);
        self
    }
}
impl Default for ScatterPlotWidgetDefinitionRequests {
    fn default() -> Self {
        Self::new()
    }
}
