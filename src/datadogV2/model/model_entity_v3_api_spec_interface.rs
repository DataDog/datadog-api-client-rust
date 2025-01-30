// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The API definition.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum EntityV3APISpecInterface {
    EntityV3APISpecInterfaceFileRef(Box<crate::datadogV2::model::EntityV3APISpecInterfaceFileRef>),
    EntityV3APISpecInterfaceDefinition(
        Box<crate::datadogV2::model::EntityV3APISpecInterfaceDefinition>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for EntityV3APISpecInterface {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::EntityV3APISpecInterfaceFileRef>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(EntityV3APISpecInterface::EntityV3APISpecInterfaceFileRef(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::EntityV3APISpecInterfaceDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(EntityV3APISpecInterface::EntityV3APISpecInterfaceDefinition(_v));
            }
        }

        return Ok(EntityV3APISpecInterface::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
