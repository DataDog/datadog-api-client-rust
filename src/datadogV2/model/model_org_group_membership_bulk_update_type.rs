// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OrgGroupMembershipBulkUpdateType {
    ORG_GROUP_MEMBERSHIP_BULK_UPDATES,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for OrgGroupMembershipBulkUpdateType {
    fn to_string(&self) -> String {
        match self {
            Self::ORG_GROUP_MEMBERSHIP_BULK_UPDATES => {
                String::from("org_group_membership_bulk_updates")
            }
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for OrgGroupMembershipBulkUpdateType {
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

impl<'de> Deserialize<'de> for OrgGroupMembershipBulkUpdateType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "org_group_membership_bulk_updates" => Self::ORG_GROUP_MEMBERSHIP_BULK_UPDATES,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
