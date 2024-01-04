// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Style customization for a top list widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToplistWidgetStyle {
    /// Top list widget display options.
    #[serde(rename = "display")]
    pub display: Option<Box<crate::datadogV1::model::ToplistWidgetDisplay>>,
    /// Top list widget scaling definition.
    #[serde(rename = "scaling")]
    pub scaling: Option<crate::datadogV1::model::ToplistWidgetScaling>,
}

impl ToplistWidgetStyle {
    pub fn new() -> ToplistWidgetStyle {
        ToplistWidgetStyle {
            display: None,
            scaling: None,
        }
    }
}
