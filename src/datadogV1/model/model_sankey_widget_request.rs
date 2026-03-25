// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Request definition for Sankey widget.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SankeyWidgetRequest {
    SankeyRumRequest(Box<crate::datadogV1::model::SankeyRumRequest>),
    SankeyNetworkRequest(Box<crate::datadogV1::model::SankeyNetworkRequest>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SankeyWidgetRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV1::model::SankeyRumRequest>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(SankeyWidgetRequest::SankeyRumRequest(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::SankeyNetworkRequest>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(SankeyWidgetRequest::SankeyNetworkRequest(_v));
            }
        }

        return Ok(SankeyWidgetRequest::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
