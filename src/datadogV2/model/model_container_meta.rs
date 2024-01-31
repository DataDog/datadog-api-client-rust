// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response metadata object.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerMeta {
    /// Paging attributes.
    #[serde(rename = "pagination")]
    pub pagination: Option<crate::datadogV2::model::ContainerMetaPage>,
}

impl ContainerMeta {
    pub fn new() -> ContainerMeta {
        ContainerMeta { pagination: None }
    }

    pub fn with_pagination(
        &mut self,
        value: crate::datadogV2::model::ContainerMetaPage,
    ) -> &mut Self {
        self.pagination = Some(value);
        self
    }
}
impl Default for ContainerMeta {
    fn default() -> Self {
        Self::new()
    }
}
