// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ServiceDefinitionSchemaVersions {
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
    #[serde(rename = "v2.1")]
    V2_1,
    #[serde(rename = "v2.2")]
    V2_2,
}

impl ToString for ServiceDefinitionSchemaVersions {
    fn to_string(&self) -> String {
        match self {
            Self::V1 => String::from("v1"),
            Self::V2 => String::from("v2"),
            Self::V2_1 => String::from("v2.1"),
            Self::V2_2 => String::from("v2.2"),
        }
    }
}
