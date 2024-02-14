// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The iframe widget allows you to embed a portion of any other web page on your dashboard. Only available on FREE layout dashboards.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IFrameWidgetDefinition {
    /// Type of the iframe widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::IFrameWidgetDefinitionType,
    /// URL of the iframe.
    #[serde(rename = "url")]
    pub url: String,
}

impl IFrameWidgetDefinition {
    pub fn new(
        type_: crate::datadogV1::model::IFrameWidgetDefinitionType,
        url: String,
    ) -> IFrameWidgetDefinition {
        IFrameWidgetDefinition { type_, url }
    }
}
