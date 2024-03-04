// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Custom links help you connect a data value to a URL, like a Datadog page or your AWS console.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetCustomLink {
    /// The flag for toggling context menu link visibility.
    #[serde(rename = "is_hidden")]
    pub is_hidden: Option<bool>,
    /// The label for the custom link URL. Keep the label short and descriptive. Use metrics and tags as variables.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// The URL of the custom link. URL must include `http` or `https`. A relative URL must start with `/`.
    #[serde(rename = "link")]
    pub link: Option<String>,
    /// The label ID that refers to a context menu link. Can be `logs`, `hosts`, `traces`, `profiles`, `processes`, `containers`, or `rum`.
    #[serde(rename = "override_label")]
    pub override_label: Option<String>,
}

impl WidgetCustomLink {
    pub fn new() -> WidgetCustomLink {
        WidgetCustomLink {
            is_hidden: None,
            label: None,
            link: None,
            override_label: None,
        }
    }

    pub fn is_hidden(mut self, value: bool) -> Self {
        self.is_hidden = Some(value);
        self
    }

    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    pub fn link(mut self, value: String) -> Self {
        self.link = Some(value);
        self
    }

    pub fn override_label(mut self, value: String) -> Self {
        self.override_label = Some(value);
        self
    }
}

impl Default for WidgetCustomLink {
    fn default() -> Self {
        Self::new()
    }
}
