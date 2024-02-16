// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The metadata associated with a request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppResponseMetadata {
    /// The time elapsed in milliseconds.
    #[serde(rename = "elapsed")]
    pub elapsed: Option<i64>,
    /// The identifier of the request.
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    /// The status of the response.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::CIAppResponseStatus>,
    /// A list of warnings (non-fatal errors) encountered. Partial results may return if
    /// warnings are present in the response.
    #[serde(rename = "warnings")]
    pub warnings: Option<Vec<crate::datadogV2::model::CIAppWarning>>,
}

impl CIAppResponseMetadata {
    pub fn new() -> CIAppResponseMetadata {
        CIAppResponseMetadata {
            elapsed: None,
            request_id: None,
            status: None,
            warnings: None,
        }
    }

    pub fn elapsed(&mut self, value: i64) -> &mut Self {
        self.elapsed = Some(value);
        self
    }

    pub fn request_id(&mut self, value: String) -> &mut Self {
        self.request_id = Some(value);
        self
    }

    pub fn status(&mut self, value: crate::datadogV2::model::CIAppResponseStatus) -> &mut Self {
        self.status = Some(value);
        self
    }

    pub fn warnings(&mut self, value: Vec<crate::datadogV2::model::CIAppWarning>) -> &mut Self {
        self.warnings = Some(value);
        self
    }
}

impl Default for CIAppResponseMetadata {
    fn default() -> Self {
        Self::new()
    }
}
