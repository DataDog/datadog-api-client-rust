// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The trigger definition for starting an investigation.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TriggerAttributes {
    MonitorAlertTrigger(Box<crate::datadogV2::model::MonitorAlertTrigger>),
    GeneralInvestigationTrigger(Box<crate::datadogV2::model::GeneralInvestigationTrigger>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for TriggerAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::MonitorAlertTrigger>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(TriggerAttributes::MonitorAlertTrigger(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::GeneralInvestigationTrigger>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(TriggerAttributes::GeneralInvestigationTrigger(_v));
            }
        }

        return Ok(TriggerAttributes::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
