// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IncidentFieldAttributesValueType {
    #[serde(rename = "multiselect")]
    MULTISELECT,
    #[serde(rename = "textarray")]
    TEXTARRAY,
    #[serde(rename = "metrictag")]
    METRICTAG,
    #[serde(rename = "autocomplete")]
    AUTOCOMPLETE,
}

impl ToString for IncidentFieldAttributesValueType {
    fn to_string(&self) -> String {
        match self {
            Self::MULTISELECT => String::from("multiselect"),
            Self::TEXTARRAY => String::from("textarray"),
            Self::METRICTAG => String::from("metrictag"),
            Self::AUTOCOMPLETE => String::from("autocomplete"),
        }
    }
}
