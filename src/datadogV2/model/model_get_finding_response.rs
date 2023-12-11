// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The expected response schema when getting a finding.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GetFindingResponse {
    /// A single finding with with message and resource configuration.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::DetailedFinding>,
}

impl GetFindingResponse {
    pub fn new(data: crate::datadogV2::model::DetailedFinding) -> GetFindingResponse {
        GetFindingResponse {
            data: Box::new(data),
        }
    }
}
