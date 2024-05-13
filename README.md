# issuu-api-rust-sdk

```rust
use issuu_api_sdk::{
    api::api_client::Api,
    drafts::controller::Draft,
    drafts::models::{CreateNewDraftRequest, DraftAccess, DraftType, Info},
};

#[tokio::main]
async fn main() {
    let draft_request = CreateNewDraftRequest {
        confirm_copyright: Some(true),
        file_url: None,
        info: Info {
            file: Some(123),
            access: Some(DraftAccess::Public),
            title: Some("My New Draft".to_string()),
            description: Some("Detailed description here".to_string()),
            preview: Some(false),
            draft_type: Some(DraftType::Book),
            show_detected_links: Some(true),
            downloadable: Some(false),
            original_publish_date: Some("2022-01-01".to_string()),
            scheduled_time: Some("2022-05-01T12:00:00Z".to_string()),
        },
    };

    let api = Api::new("https://api.issuu.com/v2");
    let draft = Draft::new(api);
    draft.create_new_draft(&draft_request).await.unwrap();
}```
