// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Process summary object.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessSummary {
    /// Attributes for a process summary.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::ProcessSummaryAttributes>,
    /// Process ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Type of process summary.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::ProcessSummaryType>,
}

impl ProcessSummary {
    pub fn new() -> ProcessSummary {
        ProcessSummary {
            attributes: None,
            id: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::ProcessSummaryAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::ProcessSummaryType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for ProcessSummary {
    fn default() -> Self {
        Self::new()
    }
}
