// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The schema resolved for the requested configuration file.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum FleetConfigFileSchemaV2ResponseData {
    FleetIntegrationSchemaDetailV2(Box<crate::datadogV2::model::FleetIntegrationSchemaDetailV2>),
    FleetConfigFileSchemaV2(Box<crate::datadogV2::model::FleetConfigFileSchemaV2>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for FleetConfigFileSchemaV2ResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::FleetIntegrationSchemaDetailV2>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(FleetConfigFileSchemaV2ResponseData::FleetIntegrationSchemaDetailV2(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::FleetConfigFileSchemaV2>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(FleetConfigFileSchemaV2ResponseData::FleetConfigFileSchemaV2(_v));
            }
        }

        return Ok(FleetConfigFileSchemaV2ResponseData::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
