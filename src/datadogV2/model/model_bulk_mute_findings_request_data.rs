// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data object containing the new bulk mute properties of the finding.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkMuteFindingsRequestData {
    /// The mute properties to be updated.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::BulkMuteFindingsRequestAttributes>,
    /// UUID to identify the request
    #[serde(rename = "id")]
    pub id: String,
    /// Meta object containing the findings to be updated.
    #[serde(rename = "meta")]
    pub meta: Box<crate::datadogV2::model::BulkMuteFindingsRequestMeta>,
    /// The JSON:API type for findings.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::FindingType,
}

impl BulkMuteFindingsRequestData {
    pub fn new(
        attributes: Box<crate::datadogV2::model::BulkMuteFindingsRequestAttributes>,
        id: String,
        meta: Box<crate::datadogV2::model::BulkMuteFindingsRequestMeta>,
        type_: crate::datadogV2::model::FindingType,
    ) -> BulkMuteFindingsRequestData {
        BulkMuteFindingsRequestData {
            attributes,
            id,
            meta,
            type_,
        }
    }
}
