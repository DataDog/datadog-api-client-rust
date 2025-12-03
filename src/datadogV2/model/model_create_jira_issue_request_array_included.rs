// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Attributes and relationships of the case linked to the Jira issue. Should contain all of the following: case, project, and security findings.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CreateJiraIssueRequestArrayIncluded {
    CreateCaseRequestData(Box<crate::datadogV2::model::CreateCaseRequestData>),
    CaseManagementProjectData(Box<crate::datadogV2::model::CaseManagementProjectData>),
    FindingData(Box<crate::datadogV2::model::FindingData>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for CreateJiraIssueRequestArrayIncluded {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::CreateCaseRequestData>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(CreateJiraIssueRequestArrayIncluded::CreateCaseRequestData(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::CaseManagementProjectData>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(CreateJiraIssueRequestArrayIncluded::CaseManagementProjectData(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::FindingData>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(CreateJiraIssueRequestArrayIncluded::FindingData(_v));
            }
        }

        return Ok(CreateJiraIssueRequestArrayIncluded::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
