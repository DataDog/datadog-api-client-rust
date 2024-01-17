// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object to map a dashboard template variable to a workflow input.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunWorkflowWidgetInput {
    /// Name of the workflow input.
    #[serde(rename = "name")]
    pub name: String,
    /// Dashboard template variable. Can be suffixed with '.value' or '.key'.
    #[serde(rename = "value")]
    pub value: String,
}

impl RunWorkflowWidgetInput {
    pub fn new(name: String, value: String) -> RunWorkflowWidgetInput {
        RunWorkflowWidgetInput { name, value }
    }
}
