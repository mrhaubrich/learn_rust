use crate::system::base::{BaseFetcher, SystemData};

// WindowsFetcher struct that implements the BaseFetcher trait
pub struct WindowsFetcher;

impl BaseFetcher for WindowsFetcher {
    fn fetch_system_data(&self) -> SystemData {
        SystemData {
            logged_user: "userX".to_string(), // Simulate fetching the logged user in Windows
        }
    }
}
