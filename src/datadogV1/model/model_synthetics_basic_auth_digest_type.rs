// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsBasicAuthDigestType {
    #[serde(rename = "digest")]
    DIGEST,
}

impl ToString for SyntheticsBasicAuthDigestType {
    fn to_string(&self) -> String {
        match self {
            Self::DIGEST => String::from("digest"),
        }
    }
}
