// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServiceDefinitionV2Dot1Version {
    #[serde(rename = "v2.1")]
    V2_1,
}

impl ToString for ServiceDefinitionV2Dot1Version {
    fn to_string(&self) -> String {
        match self {
            Self::V2_1 => String::from("v2.1"),
        }
    }
}
