// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IncidentRuleTaskIDType {
    JIRA_CREATE_ISSUE_JOB,
    NOTIFY_INCIDENT_HANDLES_JOB,
    SERVICENOW_CREATE_INCIDENT_JOB,
    SLACK_CREATE_CHANNEL_JOB,
    ZOOM_CREATE_MEETING_JOB,
    GOOGLE_MEET_CREATE_MEETING_JOB,
    WORKFLOW_AUTOMATION_JOB,
    MS_TEAMS_CREATE_MEETING_JOB,
    GOOGLE_CHAT_CREATE_SPACE_JOB,
    ZOOM_SUPPRESS_SUMMARIZATION_JOB,
    MS_TEAMS_SUPPRESS_SUMMARIZATION_JOB,
    GOOGLE_MEET_SUPPRESS_SUMMARIZATION_JOB,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for IncidentRuleTaskIDType {
    fn to_string(&self) -> String {
        match self {
            Self::JIRA_CREATE_ISSUE_JOB => String::from("jira-create-issue-job"),
            Self::NOTIFY_INCIDENT_HANDLES_JOB => String::from("notify-incident-handles-job"),
            Self::SERVICENOW_CREATE_INCIDENT_JOB => String::from("servicenow-create-incident-job"),
            Self::SLACK_CREATE_CHANNEL_JOB => String::from("slack-create-channel-job"),
            Self::ZOOM_CREATE_MEETING_JOB => String::from("zoom-create-meeting-job"),
            Self::GOOGLE_MEET_CREATE_MEETING_JOB => String::from("google-meet-create-meeting-job"),
            Self::WORKFLOW_AUTOMATION_JOB => String::from("workflow-automation-job"),
            Self::MS_TEAMS_CREATE_MEETING_JOB => String::from("ms-teams-create-meeting-job"),
            Self::GOOGLE_CHAT_CREATE_SPACE_JOB => String::from("google-chat-create-space-job"),
            Self::ZOOM_SUPPRESS_SUMMARIZATION_JOB => {
                String::from("zoom-suppress-summarization-job")
            }
            Self::MS_TEAMS_SUPPRESS_SUMMARIZATION_JOB => {
                String::from("ms-teams-suppress-summarization-job")
            }
            Self::GOOGLE_MEET_SUPPRESS_SUMMARIZATION_JOB => {
                String::from("google-meet-suppress-summarization-job")
            }
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for IncidentRuleTaskIDType {
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

impl<'de> Deserialize<'de> for IncidentRuleTaskIDType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "jira-create-issue-job" => Self::JIRA_CREATE_ISSUE_JOB,
            "notify-incident-handles-job" => Self::NOTIFY_INCIDENT_HANDLES_JOB,
            "servicenow-create-incident-job" => Self::SERVICENOW_CREATE_INCIDENT_JOB,
            "slack-create-channel-job" => Self::SLACK_CREATE_CHANNEL_JOB,
            "zoom-create-meeting-job" => Self::ZOOM_CREATE_MEETING_JOB,
            "google-meet-create-meeting-job" => Self::GOOGLE_MEET_CREATE_MEETING_JOB,
            "workflow-automation-job" => Self::WORKFLOW_AUTOMATION_JOB,
            "ms-teams-create-meeting-job" => Self::MS_TEAMS_CREATE_MEETING_JOB,
            "google-chat-create-space-job" => Self::GOOGLE_CHAT_CREATE_SPACE_JOB,
            "zoom-suppress-summarization-job" => Self::ZOOM_SUPPRESS_SUMMARIZATION_JOB,
            "ms-teams-suppress-summarization-job" => Self::MS_TEAMS_SUPPRESS_SUMMARIZATION_JOB,
            "google-meet-suppress-summarization-job" => {
                Self::GOOGLE_MEET_SUPPRESS_SUMMARIZATION_JOB
            }
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
