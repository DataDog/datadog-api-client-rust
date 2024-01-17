// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Wrapper object with the different monitor states.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorState {
    /// Dictionary where the keys are groups (comma separated lists of tags) and the values are
    /// the list of groups your monitor is broken down on.
    #[serde(rename = "groups")]
    pub groups:
        Option<std::collections::BTreeMap<String, crate::datadogV1::model::MonitorStateGroup>>,
}

impl MonitorState {
    pub fn new() -> MonitorState {
        MonitorState { groups: None }
    }
}
impl Default for MonitorState {
    fn default() -> Self {
        Self::new()
    }
}
