// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Create service definitions request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceDefinitionsCreateRequest {
    ServiceDefinitionV2Dot2(crate::datadogV2::model::ServiceDefinitionV2Dot2),
    ServiceDefinitionV2Dot1(crate::datadogV2::model::ServiceDefinitionV2Dot1),
    ServiceDefinitionV2(crate::datadogV2::model::ServiceDefinitionV2),
    ServiceDefinitionRaw(String),
}
