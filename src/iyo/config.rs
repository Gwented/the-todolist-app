use std::path;

pub struct GlobalConfig {
    pub file_path: path::PathBuf,
}

impl GlobalConfig {
    pub fn new() -> Self {
        #[cfg(target_family = "windows")]
        let home = "USERPROFILE";

        #[cfg(target_family = "unix")]
        let home = "HOME";

        let home_path = std::env::var(home).expect("Failed to fetch home variable. Let me in.");

        let mut path = path::PathBuf::from(home_path);
        path.push(".tasks.txt");

        GlobalConfig { file_path: path }
    }
}
