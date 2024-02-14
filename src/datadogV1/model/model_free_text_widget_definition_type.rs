// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FreeTextWidgetDefinitionType {
    #[serde(rename = "free_text")]
    FREE_TEXT,
}

impl ToString for FreeTextWidgetDefinitionType {
    fn to_string(&self) -> String {
        match self {
            Self::FREE_TEXT => String::from("free_text"),
        }
    }
}
