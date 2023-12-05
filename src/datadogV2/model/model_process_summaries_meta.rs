// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response metadata object.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ProcessSummariesMeta {
    /// Paging attributes.
    #[serde(rename = "page")]
    pub page: Option<Box<crate::datadogV2::model::ProcessSummariesMetaPage>>,
}

impl ProcessSummariesMeta {
    pub fn new() -> ProcessSummariesMeta {
        ProcessSummariesMeta { page: None }
    }
}