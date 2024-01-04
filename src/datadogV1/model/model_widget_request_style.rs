// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Define request widget style.
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
}
