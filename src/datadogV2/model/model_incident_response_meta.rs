// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The metadata object containing pagination metadata.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentResponseMeta {
    /// Pagination properties.
    #[serde(rename = "pagination")]
    pub pagination: Option<crate::datadogV2::model::IncidentResponseMetaPagination>,
}

impl IncidentResponseMeta {
    pub fn new() -> IncidentResponseMeta {
        IncidentResponseMeta { pagination: None }
    }

    pub fn with_pagination(
        &mut self,
        value: crate::datadogV2::model::IncidentResponseMetaPagination,
    ) -> &mut Self {
        self.pagination = Some(value);
        self
    }
}
impl Default for IncidentResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}
