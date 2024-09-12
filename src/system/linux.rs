use crate::system::base::{BaseFetcher, SystemData};

// LinuxFetcher struct that implements the BaseFetcher trait
pub struct LinuxFetcher;

impl BaseFetcher for LinuxFetcher {
    fn fetch_system_data(&self) -> SystemData {
        SystemData {
            logged_user: "linuxUser".to_string(), // Simulate fetching the logged user in Linux
        }
    }
}
