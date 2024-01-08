// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The metadata object containing pagination metadata.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentSearchResponseMeta {
    /// Pagination properties.
    #[serde(rename = "pagination")]
    pub pagination: Option<Box<crate::datadogV2::model::IncidentResponseMetaPagination>>,
}

impl IncidentSearchResponseMeta {
    pub fn new() -> IncidentSearchResponseMeta {
        IncidentSearchResponseMeta { pagination: None }
    }
}
impl Default for IncidentSearchResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}
