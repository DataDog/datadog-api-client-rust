// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The metadata object containing additional information about the list of SLOs.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SLOListResponseMetadata {
    /// The object containing information about the pages of the list of SLOs.
    #[serde(rename = "page")]
    pub page: Option<Box<crate::datadogV1::model::SLOListResponseMetadataPage>>,
}

impl SLOListResponseMetadata {
    pub fn new() -> SLOListResponseMetadata {
        SLOListResponseMetadata { page: None }
    }
}
