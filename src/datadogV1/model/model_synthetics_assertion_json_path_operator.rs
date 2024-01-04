// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsAssertionJSONPathOperator {
    #[serde(rename = "validatesJSONPath")]
    VALIDATES_JSON_PATH,
}

impl ToString for SyntheticsAssertionJSONPathOperator {
    fn to_string(&self) -> String {
        match self {
            Self::VALIDATES_JSON_PATH => String::from("validatesJSONPath"),
        }
    }
}
