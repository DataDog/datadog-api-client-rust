// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetCustomLink {
    /// The flag for toggling context menu link visibility.
    #[serde(rename = "is_hidden", skip_serializing_if = "Option::is_none")]
    pub is_hidden: bool,
    /// The label for the custom link URL. Keep the label short and descriptive. Use metrics and tags as variables.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: String,
    /// The URL of the custom link. URL must include `http` or `https`. A relative URL must start with `/`.
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: String,
    /// The label ID that refers to a context menu link. Can be `logs`, `hosts`, `traces`, `profiles`, `processes`, `containers`, or `rum`.
    #[serde(rename = "override_label", skip_serializing_if = "Option::is_none")]
    pub override_label: String,
}

