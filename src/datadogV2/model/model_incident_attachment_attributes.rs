// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The attributes object for an attachment.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IncidentAttachmentAttributes {
    IncidentAttachmentPostmortemAttributes(
        Box<crate::datadogV2::model::IncidentAttachmentPostmortemAttributes>,
    ),
    IncidentAttachmentLinkAttributes(
        Box<crate::datadogV2::model::IncidentAttachmentLinkAttributes>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for IncidentAttachmentAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::IncidentAttachmentPostmortemAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    IncidentAttachmentAttributes::IncidentAttachmentPostmortemAttributes(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::IncidentAttachmentLinkAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(IncidentAttachmentAttributes::IncidentAttachmentLinkAttributes(_v));
            }
        }

        return Ok(IncidentAttachmentAttributes::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
