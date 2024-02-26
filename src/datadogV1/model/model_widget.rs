// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Information about widget.
///
/// **Note**: The `layout` property is required for widgets in dashboards with `free` `layout_type`.
///       For the **new dashboard layout**, the `layout` property depends on the `reflow_type` of the dashboard.
///       - If `reflow_type` is `fixed`, `layout` is required.
///       - If `reflow_type` is `auto`, `layout` should not be set.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Widget {
    /// [Definition of the widget](<https://docs.datadoghq.com/dashboards/widgets/>).
    #[serde(rename = "definition")]
    pub definition: crate::datadogV1::model::WidgetDefinition,
    /// ID of the widget.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// The layout for a widget on a `free` or **new dashboard layout** dashboard.
    #[serde(rename = "layout")]
    pub layout: Option<crate::datadogV1::model::WidgetLayout>,
}

impl Widget {
    pub fn new(definition: crate::datadogV1::model::WidgetDefinition) -> Widget {
        Widget {
            definition,
            id: None,
            layout: None,
        }
    }

    pub fn id(&mut self, value: i64) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn layout(&mut self, value: crate::datadogV1::model::WidgetLayout) -> &mut Self {
        self.layout = Some(value);
        self
    }
}
