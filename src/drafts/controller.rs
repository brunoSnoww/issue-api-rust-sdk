use reqwest::Error;
use serde_json::json;

use crate::api::{
    self,
    api_client::{self, Api},
};

use super::models::{CreateNewDraftRequest, CreateNewDraftResponse};

pub struct Draft {
    base_route: &'static str,
    pub api_client: Api,
}

impl Draft {
    pub fn new(api_client: Api) -> Self {
        Draft {
            base_route: "/draft",
            api_client,
        }
    }
    pub async fn create_new_draft(
        base_route: &str,
        draft: CreateNewDraftRequest,
    ) -> Result<CreateNewDraftResponse, Error> {
        todo!()
    }
}
