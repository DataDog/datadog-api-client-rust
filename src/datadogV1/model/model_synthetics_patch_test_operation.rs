// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A single [JSON Patch](https://jsonpatch.com) operation to perform on the test
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsPatchTestOperation {
    /// The operation to perform
    #[serde(rename = "op")]
    pub op: Option<crate::datadogV1::model::SyntheticsPatchTestOperationName>,
    /// The path to the value to modify
    #[serde(rename = "path")]
    pub path: Option<String>,
    /// A value to use in a [JSON Patch](https://jsonpatch.com) operation
    #[serde(rename = "value")]
    pub value: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl SyntheticsPatchTestOperation {
    pub fn new() -> SyntheticsPatchTestOperation {
        SyntheticsPatchTestOperation {
            op: None,
            path: None,
            value: None,
        }
    }
}