// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Contains information of the host running the pipeline, stage, job, or step.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppHostInfo {
    /// FQDN of the host.
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
    /// A list of labels used to select or identify the node.
    #[serde(rename = "labels")]
    pub labels: Option<Vec<String>>,
    /// Name for the host.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The path where the code is checked out.
    #[serde(rename = "workspace")]
    pub workspace: Option<String>,
}

impl CIAppHostInfo {
    pub fn new() -> CIAppHostInfo {
        CIAppHostInfo {
            hostname: None,
            labels: None,
            name: None,
            workspace: None,
        }
    }

    pub fn hostname(&mut self, value: String) -> &mut Self {
        self.hostname = Some(value);
        self
    }

    pub fn labels(&mut self, value: Vec<String>) -> &mut Self {
        self.labels = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn workspace(&mut self, value: String) -> &mut Self {
        self.workspace = Some(value);
        self
    }
}

impl Default for CIAppHostInfo {
    fn default() -> Self {
        Self::new()
    }
}
