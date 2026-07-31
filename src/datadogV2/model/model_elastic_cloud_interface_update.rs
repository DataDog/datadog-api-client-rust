// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Partial Elastic Cloud interface for updates. Exactly one interface variant is set, selected by its `type`.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ElasticCloudInterfaceUpdate {
    ElasticCloudMonitoringInterfaceUpdate(
        Box<crate::datadogV2::model::ElasticCloudMonitoringInterfaceUpdate>,
    ),
    ElasticCloudCcmInterfaceUpdate(Box<crate::datadogV2::model::ElasticCloudCcmInterfaceUpdate>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ElasticCloudInterfaceUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ElasticCloudMonitoringInterfaceUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ElasticCloudInterfaceUpdate::ElasticCloudMonitoringInterfaceUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ElasticCloudCcmInterfaceUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ElasticCloudInterfaceUpdate::ElasticCloudCcmInterfaceUpdate(
                    _v,
                ));
            }
        }

        return Ok(ElasticCloudInterfaceUpdate::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
