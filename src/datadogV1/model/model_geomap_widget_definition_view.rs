// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The view of the world that the map should render.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeomapWidgetDefinitionView {
    /// The 2-letter ISO code of a country to focus the map on. Or `WORLD`.
    #[serde(rename = "focus")]
    pub focus: String,
}

impl GeomapWidgetDefinitionView {
    pub fn new(focus: String) -> GeomapWidgetDefinitionView {
        GeomapWidgetDefinitionView { focus }
    }
}
