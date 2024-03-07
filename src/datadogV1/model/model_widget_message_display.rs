// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WidgetMessageDisplay {
    #[serde(rename = "inline")]
    INLINE,
    #[serde(rename = "expanded-md")]
    EXPANDED_MEDIUM,
    #[serde(rename = "expanded-lg")]
    EXPANDED_LARGE,
}
impl ToString for WidgetMessageDisplay {
    fn to_string(&self) -> String {
        match self {
            Self::INLINE => String::from("inline"),
            Self::EXPANDED_MEDIUM => String::from("expanded-md"),
            Self::EXPANDED_LARGE => String::from("expanded-lg"),
        }
    }
}
