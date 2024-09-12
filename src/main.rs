mod system;

use system::factory::get_fetcher;

fn main() {
    // Get the appropriate fetcher for the current system using the factory method
    let fetcher = get_fetcher();

    // Fetch system data using the fetched fetcher
    let system_data = fetcher.fetch_system_data();
    println!("Logged User: {}", system_data.logged_user);
}
