pub enum DraftAccess {
    Public,
    Private,
}

pub enum DraftType {
    Editorial,
    Book,
    Promotional,
    Other,
}

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

pub enum DraftConversionStatus {
    Done,
    Converting,
    Failed,
}
