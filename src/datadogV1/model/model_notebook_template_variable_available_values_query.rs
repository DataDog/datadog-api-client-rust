// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Query used to dynamically populate the list of available values for the template variable.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum NotebookTemplateVariableAvailableValuesQuery {
    NotebookTemplateVariableAvailableValuesQueryLogRumSpans(
        Box<crate::datadogV1::model::NotebookTemplateVariableAvailableValuesQueryLogRumSpans>,
    ),
    NotebookTemplateVariableAvailableValuesQueryMetrics(
        Box<crate::datadogV1::model::NotebookTemplateVariableAvailableValuesQueryMetrics>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for NotebookTemplateVariableAvailableValuesQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::NotebookTemplateVariableAvailableValuesQueryLogRumSpans>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(NotebookTemplateVariableAvailableValuesQuery::NotebookTemplateVariableAvailableValuesQueryLogRumSpans(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::NotebookTemplateVariableAvailableValuesQueryMetrics>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(NotebookTemplateVariableAvailableValuesQuery::NotebookTemplateVariableAvailableValuesQueryMetrics(_v));
            }
        }

        return Ok(
            NotebookTemplateVariableAvailableValuesQuery::UnparsedObject(
                crate::datadog::UnparsedObject { value },
            ),
        );
    }
}
