use crate::api::{api_client::Api, errors::Error};

use super::models::{CreateNewDraftRequest, GetDraftBySlugResponse};

pub struct Draft {
    base_route: &'static str,
    pub api_client: Api,
}

impl Draft {
    pub fn new(api_client: Api) -> Self {
        Draft {
            base_route: "drafts",
            api_client,
        }
    }
    pub async fn create_new_draft(self, draft: &CreateNewDraftRequest) -> Result<(), Error> {
        let body = serde_json::to_string(draft)?;
        self.api_client.post(self.base_route, &body).await?;
        Ok(())
    }

    pub async fn get_draft(self, slug: &str) -> Result<GetDraftBySlugResponse, Error> {
        let url = format!("{}/{}", self.base_route, slug);
        let resp: GetDraftBySlugResponse = self.api_client.get(&url).await?;
        Ok(resp)
    }
}
