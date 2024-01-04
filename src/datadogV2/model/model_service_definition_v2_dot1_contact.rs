// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Service owner's contacts information.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceDefinitionV2Dot1Contact {
    ServiceDefinitionV2Dot1Email(Box<crate::datadogV2::model::ServiceDefinitionV2Dot1Email>),
    ServiceDefinitionV2Dot1Slack(Box<crate::datadogV2::model::ServiceDefinitionV2Dot1Slack>),
    ServiceDefinitionV2Dot1MSTeams(Box<crate::datadogV2::model::ServiceDefinitionV2Dot1MSTeams>),
}