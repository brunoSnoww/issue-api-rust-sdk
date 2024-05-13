use issuu_api_sdk::{
    api::api_client::Api,
    drafts::controller::Draft,
    drafts::models::{CreateNewDraftRequest, DraftAccess, DraftType, Info},
};

#[tokio::main]
async fn main() {
    let slug = "o3xise3e481";

    let api = Api::new("https://api.issuu.com/v2/");
    let token = "ory_at_og4PE9B1DsaXmsDWS2_qiuB_n9qIZTj8qFtDKQ7fCAw.N9ptXh7haA5wVRTQTcd8arraujLbVq46aHESJp8WgH0";
    api.set_token(&format!("{token}")).await;

    let draft = Draft::new(api);
    let p = draft.get_draft(slug).await;
    match p {
        Ok(d) => println!("{:?}", d),
        Err(e) => println!("{:?}", e),
    }
}
