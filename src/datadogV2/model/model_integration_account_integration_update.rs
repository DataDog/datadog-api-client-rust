// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Strongly-typed, per-integration partial configuration. Exactly one integration variant is set, selected by its `type`.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IntegrationAccountIntegrationUpdate {
    TwilioIntegrationUpdate(Box<crate::datadogV2::model::TwilioIntegrationUpdate>),
    ElasticCloudIntegrationUpdate(Box<crate::datadogV2::model::ElasticCloudIntegrationUpdate>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for IntegrationAccountIntegrationUpdate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::TwilioIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(IntegrationAccountIntegrationUpdate::TwilioIntegrationUpdate(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ElasticCloudIntegrationUpdate>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(IntegrationAccountIntegrationUpdate::ElasticCloudIntegrationUpdate(_v));
            }
        }

        return Ok(IntegrationAccountIntegrationUpdate::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
