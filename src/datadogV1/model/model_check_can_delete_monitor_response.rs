// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response of monitor IDs that can or can't be safely deleted.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckCanDeleteMonitorResponse {
    /// Wrapper object with the list of monitor IDs.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV1::model::CheckCanDeleteMonitorResponseData>,
    /// A mapping of Monitor ID to strings denoting where it's used.
    #[serde(rename = "errors", default, with = "::serde_with::rust::double_option")]
    pub errors: Option<Option<std::collections::BTreeMap<String, Option<Vec<String>>>>>,
}

impl CheckCanDeleteMonitorResponse {
    pub fn new(
        data: Box<crate::datadogV1::model::CheckCanDeleteMonitorResponseData>,
    ) -> CheckCanDeleteMonitorResponse {
        CheckCanDeleteMonitorResponse { data, errors: None }
    }
}
