use crate::azure::client::{AzureDevOpsClient, AzureError};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    pub id: String,
}

impl AzureDevOpsClient {
    pub async fn get_current_user(&self) -> Result<UserProfile, AzureError> {
        // Using the "me" endpoint from the profile API
        // https://app.vssps.visualstudio.com/_apis/profile/profiles/me
        self.vssps_request(Method::GET, "profile/profiles/me", None::<&String>)
            .await
    }
}
