// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServiceDefinitionV1Version {
    #[serde(rename = "v1")]
    V1,
}

impl ToString for ServiceDefinitionV1Version {
    fn to_string(&self) -> String {
        match self {
            Self::V1 => String::from("v1"),
        }
    }
}
