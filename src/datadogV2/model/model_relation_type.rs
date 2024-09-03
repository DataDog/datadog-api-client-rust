// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelationType {
    RELATIONTYPEOWNS,
    RELATIONTYPEOWNEDBY,
    RELATIONTYPEDEPENDSON,
    RELATIONTYPEDEPENDENCYOF,
    RELATIONTYPEPARTSOF,
    RELATIONTYPEHASPART,
    RELATIONTYPEOTHEROWNS,
    RELATIONTYPEOTHEROWNEDBY,
    RELATIONTYPEIMPLEMENTEDBY,
    RELATIONTYPEIMPLEMENTS,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for RelationType {
    fn to_string(&self) -> String {
        match self {
            Self::RELATIONTYPEOWNS => String::from("RelationTypeOwns"),
            Self::RELATIONTYPEOWNEDBY => String::from("RelationTypeOwnedBy"),
            Self::RELATIONTYPEDEPENDSON => String::from("RelationTypeDependsOn"),
            Self::RELATIONTYPEDEPENDENCYOF => String::from("RelationTypeDependencyOf"),
            Self::RELATIONTYPEPARTSOF => String::from("RelationTypePartsOf"),
            Self::RELATIONTYPEHASPART => String::from("RelationTypeHasPart"),
            Self::RELATIONTYPEOTHEROWNS => String::from("RelationTypeOtherOwns"),
            Self::RELATIONTYPEOTHEROWNEDBY => String::from("RelationTypeOtherOwnedBy"),
            Self::RELATIONTYPEIMPLEMENTEDBY => String::from("RelationTypeImplementedBy"),
            Self::RELATIONTYPEIMPLEMENTS => String::from("RelationTypeImplements"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for RelationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for RelationType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "RelationTypeOwns" => Self::RELATIONTYPEOWNS,
            "RelationTypeOwnedBy" => Self::RELATIONTYPEOWNEDBY,
            "RelationTypeDependsOn" => Self::RELATIONTYPEDEPENDSON,
            "RelationTypeDependencyOf" => Self::RELATIONTYPEDEPENDENCYOF,
            "RelationTypePartsOf" => Self::RELATIONTYPEPARTSOF,
            "RelationTypeHasPart" => Self::RELATIONTYPEHASPART,
            "RelationTypeOtherOwns" => Self::RELATIONTYPEOTHEROWNS,
            "RelationTypeOtherOwnedBy" => Self::RELATIONTYPEOTHEROWNEDBY,
            "RelationTypeImplementedBy" => Self::RELATIONTYPEIMPLEMENTEDBY,
            "RelationTypeImplements" => Self::RELATIONTYPEIMPLEMENTS,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
