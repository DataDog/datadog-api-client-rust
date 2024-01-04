// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Paging attributes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerImageMetaPage {
    /// The cursor used to get the current results, if any.
    #[serde(rename = "cursor")]
    pub cursor: Option<String>,
    /// Number of results returned
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
    /// The cursor used to get the next results, if any.
    #[serde(rename = "next_cursor")]
    pub next_cursor: Option<String>,
    /// The cursor used to get the previous results, if any.
    #[serde(
        rename = "prev_cursor",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub prev_cursor: Option<Option<String>>,
    /// Total number of records that match the query.
    #[serde(rename = "total")]
    pub total: Option<i64>,
    /// Type of Container Image pagination.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ContainerImageMetaPageType>,
}

impl ContainerImageMetaPage {
    pub fn new() -> ContainerImageMetaPage {
        ContainerImageMetaPage {
            cursor: None,
            limit: None,
            next_cursor: None,
            prev_cursor: None,
            total: None,
            type_: None,
        }
    }
}
