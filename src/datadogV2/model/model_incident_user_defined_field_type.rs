// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum IncidentUserDefinedFieldType {
    #[serde(rename = "user_defined_field")]
    USER_DEFINED_FIELD,
}

impl ToString for IncidentUserDefinedFieldType {
    fn to_string(&self) -> String {
        match self {
            Self::USER_DEFINED_FIELD => String::from("user_defined_field"),
        }
    }
}
