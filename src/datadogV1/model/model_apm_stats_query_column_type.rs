// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Column properties.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApmStatsQueryColumnType {
    /// A user-assigned alias for the column.
    #[serde(rename = "alias")]
    pub alias: Option<String>,
    /// Define a display mode for the table cell.
    #[serde(rename = "cell_display_mode")]
    pub cell_display_mode: Option<crate::datadogV1::model::TableWidgetCellDisplayMode>,
    /// Column name.
    #[serde(rename = "name")]
    pub name: String,
    /// Widget sorting methods.
    #[serde(rename = "order")]
    pub order: Option<crate::datadogV1::model::WidgetSort>,
}

impl ApmStatsQueryColumnType {
    pub fn new(name: String) -> ApmStatsQueryColumnType {
        ApmStatsQueryColumnType {
            alias: None,
            cell_display_mode: None,
            name,
            order: None,
        }
    }

    pub fn alias(mut self, value: String) -> Self {
        self.alias = Some(value);
        self
    }

    pub fn cell_display_mode(
        mut self,
        value: crate::datadogV1::model::TableWidgetCellDisplayMode,
    ) -> Self {
        self.cell_display_mode = Some(value);
        self
    }

    pub fn order(mut self, value: crate::datadogV1::model::WidgetSort) -> Self {
        self.order = Some(value);
        self
    }
}
