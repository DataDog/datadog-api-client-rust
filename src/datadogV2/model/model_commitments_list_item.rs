// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A commitment item, which varies based on the provider, product, and commitment type.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CommitmentsListItem {
    CommitmentsAwsEC2RICommitment(Box<crate::datadogV2::model::CommitmentsAwsEC2RICommitment>),
    CommitmentsAwsRDSRICommitment(Box<crate::datadogV2::model::CommitmentsAwsRDSRICommitment>),
    CommitmentsAwsElasticacheRICommitment(
        Box<crate::datadogV2::model::CommitmentsAwsElasticacheRICommitment>,
    ),
    CommitmentsAwsSPCommitment(Box<crate::datadogV2::model::CommitmentsAwsSPCommitment>),
    CommitmentsAzureVMRICommitment(Box<crate::datadogV2::model::CommitmentsAzureVMRICommitment>),
    CommitmentsAzureComputeSPCommitment(
        Box<crate::datadogV2::model::CommitmentsAzureComputeSPCommitment>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for CommitmentsListItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CommitmentsAwsEC2RICommitment>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CommitmentsListItem::CommitmentsAwsEC2RICommitment(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CommitmentsAwsRDSRICommitment>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CommitmentsListItem::CommitmentsAwsRDSRICommitment(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CommitmentsAwsElasticacheRICommitment>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CommitmentsListItem::CommitmentsAwsElasticacheRICommitment(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CommitmentsAwsSPCommitment>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CommitmentsListItem::CommitmentsAwsSPCommitment(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CommitmentsAzureVMRICommitment>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CommitmentsListItem::CommitmentsAzureVMRICommitment(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CommitmentsAzureComputeSPCommitment>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CommitmentsListItem::CommitmentsAzureComputeSPCommitment(_v));
            }
        }

        return Ok(CommitmentsListItem::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
