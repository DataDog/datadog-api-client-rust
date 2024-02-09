// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Powerpack response metadata.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerpacksResponseMeta {
    /// Powerpack response pagination metadata.
    #[serde(rename = "pagination")]
    pub pagination: Option<crate::datadogV2::model::PowerpacksResponseMetaPagination>,
}

impl PowerpacksResponseMeta {
    pub fn new() -> PowerpacksResponseMeta {
        PowerpacksResponseMeta { pagination: None }
    }

    pub fn pagination(
        &mut self,
        value: crate::datadogV2::model::PowerpacksResponseMetaPagination,
    ) -> &mut Self {
        self.pagination = Some(value);
        self
    }
}

impl Default for PowerpacksResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}
