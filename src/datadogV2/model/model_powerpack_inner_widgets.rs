// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Powerpack group widget definition of individual widgets.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerpackInnerWidgets {
    /// Information about widget.
    #[serde(rename = "definition")]
    pub definition: std::collections::BTreeMap<String, serde_json::Value>,
    /// Powerpack inner widget layout.
    #[serde(rename = "layout")]
    pub layout: Option<crate::datadogV2::model::PowerpackInnerWidgetLayout>,
}

impl PowerpackInnerWidgets {
    pub fn new(
        definition: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> PowerpackInnerWidgets {
        PowerpackInnerWidgets {
            definition,
            layout: None,
        }
    }

    pub fn with_layout(
        &mut self,
        value: crate::datadogV2::model::PowerpackInnerWidgetLayout,
    ) -> &mut Self {
        self.layout = Some(value);
        self
    }
}
