// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorState {
    /// Dictionary where the keys are groups (comma separated lists of tags) and the values are
the list of groups your monitor is broken down on.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: map[string]MonitorStateGroup,
}

