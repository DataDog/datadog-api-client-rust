// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Powerpack template variables.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerpackTemplateVariables {
    /// Template variables controlled at the powerpack level.
    #[serde(rename = "controlled_by_powerpack")]
    pub controlled_by_powerpack:
        Option<Vec<crate::datadogV1::model::PowerpackTemplateVariableContents>>,
    /// Template variables controlled by the external resource, such as the dashboard this powerpack is on.
    #[serde(rename = "controlled_externally")]
    pub controlled_externally:
        Option<Vec<crate::datadogV1::model::PowerpackTemplateVariableContents>>,
}

impl PowerpackTemplateVariables {
    pub fn new() -> PowerpackTemplateVariables {
        PowerpackTemplateVariables {
            controlled_by_powerpack: None,
            controlled_externally: None,
        }
    }
}
