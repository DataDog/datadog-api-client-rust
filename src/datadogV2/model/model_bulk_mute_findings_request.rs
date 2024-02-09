// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The new bulk mute finding request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkMuteFindingsRequest {
    /// Data object containing the new bulk mute properties of the finding.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::BulkMuteFindingsRequestData,
}

impl BulkMuteFindingsRequest {
    pub fn new(
        data: crate::datadogV2::model::BulkMuteFindingsRequestData,
    ) -> BulkMuteFindingsRequest {
        BulkMuteFindingsRequest { data }
    }
}
