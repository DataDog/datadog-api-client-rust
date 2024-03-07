// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LogsLookupProcessorType {
    #[serde(rename = "lookup-processor")]
    LOOKUP_PROCESSOR,
}

impl ToString for LogsLookupProcessorType {
    fn to_string(&self) -> String {
        match self {
            Self::LOOKUP_PROCESSOR => String::from("lookup-processor"),
        }
    }
}
