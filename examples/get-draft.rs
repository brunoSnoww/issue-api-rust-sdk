use issuu_api_sdk::{api::api_client::Api, drafts::controller::Draft};

#[tokio::main]
async fn main() {
    let slug = "o3xise3e481";

    let api = Api::new("https://api.issuu.com/v2/");
    /* set token
        let token = "";
        api.set_token(&format!("{token}")).await;
    */
    let draft = Draft::new(api);
    let p = draft.get_draft(slug).await;
    match p {
        Ok(d) => println!("{:?}", d),
        Err(e) => println!("{:?}", e),
    }
}
