// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeomapWidgetDefinitionView {
    /// The 2-letter ISO code of a country to focus the map on. Or `WORLD`.
    #[serde(rename = "focus", skip_serializing_if = "Option::is_none")]
    pub focus: String,
}

