// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing meta attributes of response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseMetaAttributes {
    /// Pagination object.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::Pagination>,
}

impl ResponseMetaAttributes {
    pub fn new() -> ResponseMetaAttributes {
        ResponseMetaAttributes { page: None }
    }

    pub fn page(&mut self, value: crate::datadogV2::model::Pagination) -> &mut Self {
        self.page = Some(value);
        self
    }
}

impl Default for ResponseMetaAttributes {
    fn default() -> Self {
        Self::new()
    }
}
