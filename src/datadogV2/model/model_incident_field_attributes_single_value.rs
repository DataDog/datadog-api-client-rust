// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A field with a single value selected.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentFieldAttributesSingleValue {
    /// Type of the single value field definitions.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::IncidentFieldAttributesSingleValueType>,
    /// The single value selected for this field.
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option")]
    pub value: Option<Option<String>>,
}

impl IncidentFieldAttributesSingleValue {
    pub fn new() -> IncidentFieldAttributesSingleValue {
        IncidentFieldAttributesSingleValue {
            type_: None,
            value: None,
        }
    }

    pub fn type_(
        &mut self,
        value: crate::datadogV2::model::IncidentFieldAttributesSingleValueType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }

    pub fn value(&mut self, value: Option<String>) -> &mut Self {
        self.value = Some(value);
        self
    }
}

impl Default for IncidentFieldAttributesSingleValue {
    fn default() -> Self {
        Self::new()
    }
}
