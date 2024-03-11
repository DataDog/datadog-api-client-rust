// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Possible Container Image models.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ContainerImageItem {
    ContainerImage(Box<crate::datadogV2::model::ContainerImage>),
    ContainerImageGroup(Box<crate::datadogV2::model::ContainerImageGroup>),
    UnparsedObject(crate::datadog::UnparsedObejct),
}

impl<'de> Deserialize<'de> for ContainerImageItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::ContainerImage>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ContainerImageItem::ContainerImage(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::ContainerImageGroup>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(ContainerImageItem::ContainerImageGroup(_v));
            }
        }

        return Ok(ContainerImageItem::UnparsedObject(
            crate::datadog::UnparsedObejct { value },
        ));
    }
}
