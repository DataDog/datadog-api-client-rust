// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The mute properties to be updated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkMuteFindingsRequestAttributes {
    /// Object containing the new mute properties of the findings.
    #[serde(rename = "mute")]
    pub mute: crate::datadogV2::model::BulkMuteFindingsRequestProperties,
}

impl BulkMuteFindingsRequestAttributes {
    pub fn new(
        mute: crate::datadogV2::model::BulkMuteFindingsRequestProperties,
    ) -> BulkMuteFindingsRequestAttributes {
        BulkMuteFindingsRequestAttributes { mute }
    }
}
