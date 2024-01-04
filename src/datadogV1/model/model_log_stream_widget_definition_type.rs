// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogStreamWidgetDefinitionType {
    #[serde(rename = "log_stream")]
    LOG_STREAM,
}

impl ToString for LogStreamWidgetDefinitionType {
    fn to_string(&self) -> String {
        match self {
            Self::LOG_STREAM => String::from("log_stream"),
        }
    }
}
