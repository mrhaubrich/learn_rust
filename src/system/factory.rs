use crate::system::base::BaseFetcher;
use crate::system::linux::LinuxFetcher;
use crate::system::windows::WindowsFetcher;

pub fn get_fetcher() -> Box<dyn BaseFetcher> {
    match std::env::consts::OS {
        "linux" => Box::new(LinuxFetcher),
        "windows" => Box::new(WindowsFetcher),
        _ => panic!("Unsupported operating system"),
    }
}
