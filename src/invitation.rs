use crate::{authority::Authority, client, errors::*};
use reqwest::Method;
use std::collections::HashMap;

/// An invitation
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invitation {
    pub email: String,
    pub authority: Authority,
}

#[cfg(test)]
mod tests {
    use crate::invitation::*;
    use serde_json;

    fn invitation_example1() -> Invitation {
        Invitation {
            email: "example1@example.com".to_string(),
            authority: Authority::Manager,
        }
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "email": "example1@example.com",
            "authority": "manager"
        })
    }

    fn invitation_example2() -> Invitation {
        Invitation {
            email: "example2@example.com".to_string(),
            authority: Authority::Collaborator,
        }
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "email": "example2@example.com",
            "authority": "collaborator"
        })
    }

    fn invitation_example3() -> Invitation {
        Invitation {
            email: "example3@example.com".to_string(),
            authority: Authority::Viewer,
        }
    }

    fn json_example3() -> serde_json::Value {
        json!({
            "email": "example3@example.com",
            "authority": "viewer"
        })
    }

    #[test]
    fn serialize_invitation() {
        assert_eq!(
            json_example1(),
            serde_json::to_value(&invitation_example1()).unwrap()
        );
        assert_eq!(
            json_example2(),
            serde_json::to_value(&invitation_example2()).unwrap()
        );
        assert_eq!(
            json_example3(),
            serde_json::to_value(&invitation_example3()).unwrap()
        );
    }

    #[test]
    fn deserialize_invitation() {
        assert_eq!(
            invitation_example1(),
            serde_json::from_value(json_example1()).unwrap()
        );
        assert_eq!(
            invitation_example2(),
            serde_json::from_value(json_example2()).unwrap()
        );
        assert_eq!(
            invitation_example3(),
            serde_json::from_value(json_example3()).unwrap()
        );
    }
}

impl client::Client {
    /// Creates a new invitation.
    ///
    /// See https://mackerel.io/api-docs/entry/invitations#create.
    pub async fn create_invitation(&self, invitation: Invitation) -> Result<Invitation> {
        self.request(
            Method::POST,
            "/api/v0/invitations",
            vec![],
            Some(invitation),
            |res: Invitation| res,
        )
        .await
    }

    /// Revokes an invitation.
    ///
    /// See https://mackerel.io/api-docs/entry/invitations#revoke.
    pub async fn revoke_invitation(&self, email: &str) -> Result<()> {
        let body: HashMap<&str, &str> = [("email", email)].iter().cloned().collect();
        self.request(
            Method::POST,
            "/api/v0/invitations/revoke",
            vec![],
            Some(body),
            |_: HashMap<String, bool>| (),
        )
        .await
    }
}
