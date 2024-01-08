// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object containing the query content.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansListRequestData {
    /// The object containing all the query parameters.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::SpansListRequestAttributes>>,
    /// The type of resource. The value should always be search_request.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SpansListRequestType>,
}

impl SpansListRequestData {
    pub fn new() -> SpansListRequestData {
        SpansListRequestData {
            attributes: None,
            type_: None,
        }
    }
}
impl Default for SpansListRequestData {
    fn default() -> Self {
        Self::new()
    }
}
