pub struct SystemData {
    pub logged_user: String,
}

pub trait BaseFetcher {
    fn fetch_system_data(&self) -> SystemData;
}
