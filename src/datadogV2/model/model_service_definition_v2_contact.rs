// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Service owner's contacts information.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceDefinitionV2Contact {
    ServiceDefinitionV2Email(crate::datadogV2::model::ServiceDefinitionV2Email),
    ServiceDefinitionV2Slack(crate::datadogV2::model::ServiceDefinitionV2Slack),
    ServiceDefinitionV2MSTeams(crate::datadogV2::model::ServiceDefinitionV2MSTeams),
}
