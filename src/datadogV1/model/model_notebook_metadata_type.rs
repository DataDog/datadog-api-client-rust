// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum NotebookMetadataType {
    #[serde(rename = "postmortem")]
    POSTMORTEM,
    #[serde(rename = "runbook")]
    RUNBOOK,
    #[serde(rename = "investigation")]
    INVESTIGATION,
    #[serde(rename = "documentation")]
    DOCUMENTATION,
    #[serde(rename = "report")]
    REPORT,
}
impl ToString for NotebookMetadataType {
    fn to_string(&self) -> String {
        match self {
            Self::POSTMORTEM => String::from("postmortem"),
            Self::RUNBOOK => String::from("runbook"),
            Self::INVESTIGATION => String::from("investigation"),
            Self::DOCUMENTATION => String::from("documentation"),
            Self::REPORT => String::from("report"),
        }
    }
}
