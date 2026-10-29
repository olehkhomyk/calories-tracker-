use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    pub data: T
}