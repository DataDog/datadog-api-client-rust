// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServiceDefinitionV2Dot1LinkType {
    #[serde(rename = "doc")]
    DOC,
    #[serde(rename = "repo")]
    REPO,
    #[serde(rename = "runbook")]
    RUNBOOK,
    #[serde(rename = "dashboard")]
    DASHBOARD,
    #[serde(rename = "other")]
    OTHER,
}

impl ToString for ServiceDefinitionV2Dot1LinkType {
    fn to_string(&self) -> String {
        match self {
            Self::DOC => String::from("doc"),
            Self::REPO => String::from("repo"),
            Self::RUNBOOK => String::from("runbook"),
            Self::DASHBOARD => String::from("dashboard"),
            Self::OTHER => String::from("other"),
        }
    }
}
