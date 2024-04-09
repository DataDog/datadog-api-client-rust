// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Relationship of AuthN Mapping create object to a Role or Team.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum AuthNMappingCreateRelationships {
    AuthNMappingRelationshipToRole(Box<crate::datadogV2::model::AuthNMappingRelationshipToRole>),
    AuthNMappingRelationshipToTeam(Box<crate::datadogV2::model::AuthNMappingRelationshipToTeam>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for AuthNMappingCreateRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::AuthNMappingRelationshipToRole>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(AuthNMappingCreateRelationships::AuthNMappingRelationshipToRole(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::AuthNMappingRelationshipToTeam>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(AuthNMappingCreateRelationships::AuthNMappingRelationshipToTeam(_v));
            }
        }

        return Ok(AuthNMappingCreateRelationships::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
