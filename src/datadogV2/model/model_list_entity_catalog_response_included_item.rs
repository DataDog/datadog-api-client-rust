// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// List entity response included item.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ListEntityCatalogResponseIncludedItem {
    EntityResponseIncludedSchema(Box<crate::datadogV2::model::EntityResponseIncludedSchema>),
    EntityResponseIncludedRawSchema(Box<crate::datadogV2::model::EntityResponseIncludedRawSchema>),
    EntityResponseIncludedRelatedEntity(
        Box<crate::datadogV2::model::EntityResponseIncludedRelatedEntity>,
    ),
    EntityResponseIncludedOncall(Box<crate::datadogV2::model::EntityResponseIncludedOncall>),
    EntityResponseIncludedIncident(Box<crate::datadogV2::model::EntityResponseIncludedIncident>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ListEntityCatalogResponseIncludedItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::EntityResponseIncludedSchema>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ListEntityCatalogResponseIncludedItem::EntityResponseIncludedSchema(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::EntityResponseIncludedRawSchema>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ListEntityCatalogResponseIncludedItem::EntityResponseIncludedRawSchema(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::EntityResponseIncludedRelatedEntity>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ListEntityCatalogResponseIncludedItem::EntityResponseIncludedRelatedEntity(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::EntityResponseIncludedOncall>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ListEntityCatalogResponseIncludedItem::EntityResponseIncludedOncall(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::EntityResponseIncludedIncident>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ListEntityCatalogResponseIncludedItem::EntityResponseIncludedIncident(_v),
                );
            }
        }

        return Ok(ListEntityCatalogResponseIncludedItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
