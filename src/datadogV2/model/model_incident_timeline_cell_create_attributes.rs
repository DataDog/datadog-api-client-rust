// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The timeline cell's attributes for a create request.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IncidentTimelineCellCreateAttributes {
    IncidentTimelineCellMarkdownCreateAttributes(
        Box<crate::datadogV2::model::IncidentTimelineCellMarkdownCreateAttributes>,
    ),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for IncidentTimelineCellCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::IncidentTimelineCellMarkdownCreateAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(IncidentTimelineCellCreateAttributes::IncidentTimelineCellMarkdownCreateAttributes(_v));
            }
        }

        return Ok(IncidentTimelineCellCreateAttributes::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
