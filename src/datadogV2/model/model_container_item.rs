// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Possible Container models.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ContainerItem {
    Container(Box<crate::datadogV2::model::Container>),
    ContainerGroup(Box<crate::datadogV2::model::ContainerGroup>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ContainerItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::Container>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ContainerItem::Container(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::ContainerGroup>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ContainerItem::ContainerGroup(_v));
            }
        }

        return Ok(ContainerItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
