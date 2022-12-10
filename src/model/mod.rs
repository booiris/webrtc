#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Payload {
    pub message: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ClientReq {
    pub client: IdStruct,
    pub aim_user: i64,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct IdStruct {
    pub id: i64,
    pub ip: String,
    pub port: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ClientResp {
    pub aim_user: Option<IdStruct>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct DbData {
    pub id_data: IdStruct,
}
