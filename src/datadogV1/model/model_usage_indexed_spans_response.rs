// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A response containing indexed spans usage.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageIndexedSpansResponse {
    /// Array with the number of hourly traces indexed for a given organization.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageIndexedSpansHour>>,
}

impl UsageIndexedSpansResponse {
    pub fn new() -> UsageIndexedSpansResponse {
        UsageIndexedSpansResponse { usage: None }
    }
}
