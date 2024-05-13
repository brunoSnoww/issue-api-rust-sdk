use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DraftAccess {
    Public,
    Private,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DraftType {
    Editorial,
    Book,
    Promotional,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DraftFileInfoType {
    Unknown,
    Doc,
    Odp,
    Odt,
    Pdf,
    Ppt,
    Rtf,
    Sxi,
    Sxw,
    Wpd,
    Epub,
    Mobi,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DraftConversionStatus {
    Done,
    Converting,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNewDraftRequest {
    pub confirm_copyright: Option<bool>,
    pub file_url: Option<String>,
    pub info: Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub file: Option<i32>,
    pub access: Option<DraftAccess>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub preview: Option<bool>,
    pub draft_type: Option<DraftType>,
    pub show_detected_links: Option<bool>,
    pub downloadable: Option<bool>,
    pub original_publish_date: Option<String>,
    pub scheduled_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageSize {
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cover {
    pub small: ImageSize,
    pub medium: ImageSize,
    pub large: ImageSize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub name: String,
    pub file_type: DraftFileInfoType,
    pub size: i64,
    pub page_count: i32,
    pub conversion_status: DraftConversionStatus,
    pub is_copyright_confirmed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Changes {
    pub file: Option<i32>,
    pub access: Option<DraftAccess>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub preview: Option<bool>,
    pub draft_type: Option<DraftType>,
    pub show_detected_links: Option<bool>,
    pub downloadable: Option<bool>,
    pub original_publish_date: Option<String>,
    pub scheduled_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNewDraftResponse {
    pub slug: String,
    pub owner: String,
    pub cover: Cover,
    pub file_info: Option<FileInfo>,
    pub state: Option<String>, // Using String to simplify; ideally, this should be an enum if only limited values are allowed.
    pub location: String,
    pub changes: Option<Changes>,
    pub created: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum DraftState {
    Draft,
    Published,
    Scheduled,
    Unpublished,
    Quarantined,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetDraftBySlugResponse {
    #[serde(flatten)]
    pub base: CreateNewDraftResponse,
    pub state: DraftState,
}
