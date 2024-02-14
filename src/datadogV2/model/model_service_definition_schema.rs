// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Service definition schema.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceDefinitionSchema {
    ServiceDefinitionV1(Box<crate::datadogV2::model::ServiceDefinitionV1>),
    ServiceDefinitionV2(Box<crate::datadogV2::model::ServiceDefinitionV2>),
    ServiceDefinitionV2Dot1(Box<crate::datadogV2::model::ServiceDefinitionV2Dot1>),
    ServiceDefinitionV2Dot2(Box<crate::datadogV2::model::ServiceDefinitionV2Dot2>),
}
