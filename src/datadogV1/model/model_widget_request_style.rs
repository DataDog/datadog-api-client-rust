// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Define request widget style.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetRequestStyle {
    /// Type of lines displayed.
    #[serde(rename = "line_type")]
    pub line_type: Option<crate::datadogV1::model::WidgetLineType>,
    /// Width of line displayed.
    #[serde(rename = "line_width")]
    pub line_width: Option<crate::datadogV1::model::WidgetLineWidth>,
    /// Color palette to apply to the widget.
    #[serde(rename = "palette")]
    pub palette: Option<String>,
}

impl WidgetRequestStyle {
    pub fn new() -> WidgetRequestStyle {
        WidgetRequestStyle {
            line_type: None,
            line_width: None,
            palette: None,
        }
    }

    pub fn line_type(&mut self, value: crate::datadogV1::model::WidgetLineType) -> &mut Self {
        self.line_type = Some(value);
        self
    }

    pub fn line_width(&mut self, value: crate::datadogV1::model::WidgetLineWidth) -> &mut Self {
        self.line_width = Some(value);
        self
    }

    pub fn palette(&mut self, value: String) -> &mut Self {
        self.palette = Some(value);
        self
    }
}

impl Default for WidgetRequestStyle {
    fn default() -> Self {
        Self::new()
    }
}
