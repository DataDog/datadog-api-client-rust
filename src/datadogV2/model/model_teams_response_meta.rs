// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Teams response metadata.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamsResponseMeta {
    /// Teams response metadata.
    #[serde(rename = "pagination")]
    pub pagination: Option<crate::datadogV2::model::TeamsResponseMetaPagination>,
}

impl TeamsResponseMeta {
    pub fn new() -> TeamsResponseMeta {
        TeamsResponseMeta { pagination: None }
    }

    pub fn with_pagination(
        &mut self,
        value: crate::datadogV2::model::TeamsResponseMetaPagination,
    ) -> &mut Self {
        self.pagination = Some(value);
        self
    }
}
impl Default for TeamsResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}
