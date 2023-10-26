// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ProcessSummariesMeta {
    /// Paging attributes.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<Box<crate::datadogV2::model::ProcessSummariesMetaPage>>,
}

impl ProcessSummariesMeta {
    /// Response metadata object.
    pub fn new() -> ProcessSummariesMeta {
        ProcessSummariesMeta { page: None }
    }
}
