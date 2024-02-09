// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A field with potentially multiple values selected.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentFieldAttributesMultipleValue {
    /// Type of the multiple value field definitions.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::IncidentFieldAttributesValueType>,
    /// The multiple values selected for this field.
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option")]
    pub value: Option<Option<Vec<String>>>,
}

impl IncidentFieldAttributesMultipleValue {
    pub fn new() -> IncidentFieldAttributesMultipleValue {
        IncidentFieldAttributesMultipleValue {
            type_: None,
            value: None,
        }
    }

    pub fn type_(
        &mut self,
        value: crate::datadogV2::model::IncidentFieldAttributesValueType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }

    pub fn value(&mut self, value: Option<Vec<String>>) -> &mut Self {
        self.value = Some(value);
        self
    }
}

impl Default for IncidentFieldAttributesMultipleValue {
    fn default() -> Self {
        Self::new()
    }
}
