// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServiceDefinitionV2LinkType {
    #[serde(rename = "doc")]
    DOC,
    #[serde(rename = "wiki")]
    WIKI,
    #[serde(rename = "runbook")]
    RUNBOOK,
    #[serde(rename = "url")]
    URL,
    #[serde(rename = "repo")]
    REPO,
    #[serde(rename = "dashboard")]
    DASHBOARD,
    #[serde(rename = "oncall")]
    ONCALL,
    #[serde(rename = "code")]
    CODE,
    #[serde(rename = "link")]
    LINK,
}

impl ToString for ServiceDefinitionV2LinkType {
    fn to_string(&self) -> String {
        match self {
            Self::DOC => String::from("doc"),
            Self::WIKI => String::from("wiki"),
            Self::RUNBOOK => String::from("runbook"),
            Self::URL => String::from("url"),
            Self::REPO => String::from("repo"),
            Self::DASHBOARD => String::from("dashboard"),
            Self::ONCALL => String::from("oncall"),
            Self::CODE => String::from("code"),
            Self::LINK => String::from("link"),
        }
    }
}
