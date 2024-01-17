// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object containing all the query parameters.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansListRequestAttributes {
    /// The search and filter query settings.
    #[serde(rename = "filter")]
    pub filter: Option<Box<crate::datadogV2::model::SpansQueryFilter>>,
    /// Global query options that are used during the query.
    /// Note: You should only supply timezone or time offset but not both otherwise the query will fail.
    #[serde(rename = "options")]
    pub options: Option<Box<crate::datadogV2::model::SpansQueryOptions>>,
    /// Paging attributes for listing spans.
    #[serde(rename = "page")]
    pub page: Option<Box<crate::datadogV2::model::SpansListRequestPage>>,
    /// Sort parameters when querying spans.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::SpansSort>,
}

impl SpansListRequestAttributes {
    pub fn new() -> SpansListRequestAttributes {
        SpansListRequestAttributes {
            filter: None,
            options: None,
            page: None,
            sort: None,
        }
    }
}
impl Default for SpansListRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}
