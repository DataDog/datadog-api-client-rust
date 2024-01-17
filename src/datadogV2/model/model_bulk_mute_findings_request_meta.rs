// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Meta object containing the findings to be updated.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkMuteFindingsRequestMeta {
    /// Array of findings.
    #[serde(rename = "findings")]
    pub findings: Option<Vec<crate::datadogV2::model::BulkMuteFindingsRequestMetaFindings>>,
}

impl BulkMuteFindingsRequestMeta {
    pub fn new() -> BulkMuteFindingsRequestMeta {
        BulkMuteFindingsRequestMeta { findings: None }
    }
}
