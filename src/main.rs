use std::ffi::OsStr;
use std::fmt::{format, Debug};
use std::fs;
use std::io::stdin;
use std::path::Path;
use crossterm::execute;
use glob::{glob, GlobResult, Paths, PatternError};
use inquire::formatter::MultiOptionFormatter;
use inquire::list_option::ListOption;
use inquire::MultiSelect;
use inquire::validator::Validation;
use tabled::{Table, Tabled};

#[derive(PartialEq, Tabled)]
struct Cleared {
    Program: String,

}
impl PartialEq<Option<Cleared>> for &Cleared {
    fn eq(&self, other: &Option<Cleared>) -> bool {
        match other {
            Some(other) => return other.Program.eq(&*self.Program),
            None => return false,
        }
    }
}
struct CleanerData {
    pub path: String,
    pub category: String,
    pub program: String,

    pub files_to_remove: Vec<String>,
    pub folders_to_remove: Vec<String>,
    pub directories_to_remove: Vec<String>,

    pub remove_all_in_dir: bool,
    pub remove_directory_after_clean: bool,
    pub remove_directories: bool,
    pub remove_files: bool
}
fn get_steam_directory() {

}

fn get_file_size_string(size: u64) -> String {
    if size <= 0 {
        return "0 B".to_string();
    }

    let units = ["B", "KB", "MB", "GB", "TB"];
    let digit_groups = ((size as f64).log(1024.0)).floor() as usize;

    let size_in_units = size as f64 / 1024_f64.powi(digit_groups as i32);
    format!("{:.1} {}", size_in_units, units[digit_groups])
}


fn main() {
    execute!(
        std::io::stdout(),
        crossterm::terminal::SetTitle("WinBooster CLI v1.0.2")
    );

    let username = &*whoami::username();
    let mut database: Vec<CleanerData> = Vec::new();

    let mut options: Vec<&str> = vec![];

    //<editor-fold desc="Windows">
    let c_windows_debug_wia = CleanerData {
        path: "C:\\Windows\\debug\\WIA\\*".parse().unwrap(),
        program: "Windows".parse().unwrap(),
        files_to_remove: vec!["wiatrace.log".parse().unwrap()],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![]
    };
    database.push(c_windows_debug_wia);
    let c_windows_prefetch = CleanerData {
        path: "C:\\Windows\\Prefetch\\*".parse().unwrap(),
        program: "Windows".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![]
    };
    database.push(c_windows_prefetch);
    let c_windows_dumps = CleanerData {
        path: "C:\\Windows\\Minidump\\*".parse().unwrap(),
        program: "Windows".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![]
    };
    database.push(c_windows_dumps);
    let c_windows_security_logs = CleanerData {
        path: "C:\\Windows\\security\\logs\\*.log".parse().unwrap(),
        program: "Windows".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(), remove_directories: false,
        remove_files: true, directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![]
    };
    database.push(c_windows_security_logs);
    let c_windows_security_database_logs = CleanerData {
        path: "C:\\Windows\\security\\database\\*.log".parse().unwrap(),
        program: "Windows".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![]
    };
    database.push(c_windows_security_database_logs);

    let c_temp = CleanerData {
        path: "C:\\Temp\\*".parse().unwrap(),
        program: "Windows".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![]
    };
    database.push(c_temp);
    let c_windows_panther = CleanerData {
        path: "C:\\Windows\\Panther".parse().unwrap(),
        program: "Windows".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false, directories_to_remove: vec![],
        remove_all_in_dir: true,
        remove_directory_after_clean: false,
        folders_to_remove: vec![]
    };
    database.push(c_windows_panther);
    let c_windows_temp = CleanerData {
        path: "C:\\Windows\\Temp\\*".parse().unwrap(),
        program: "Windows".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![]
    };
    database.push(c_windows_temp);
    let c_windows_logs = CleanerData {
        path: "C:\\Windows\\Logs\\*".parse().unwrap(),
        program: "Windows".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![]
    };
    database.push(c_windows_logs);
    let c_windows_logs_windows_update = CleanerData {
        path: "C:\\Windows\\Logs\\WindowsUpdate\\*".parse().unwrap(),
        program: "Windows".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![]
    };
    database.push(c_windows_logs_windows_update);
    let c_users_appdata_local_temp = CleanerData {
        path: "C:\\Users\\{username}\\AppData\\Local\\Temp\\*".parse().unwrap(),
        program: "Windows".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![]
    };
    database.push(c_users_appdata_local_temp);
    let c_programdata_usoshared_logs = CleanerData {
        path: "C:\\ProgramData\\USOShared\\Logs\\*".parse().unwrap(),
        program: "Windows".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![]
    };
    database.push(c_programdata_usoshared_logs);
    let c_users_appdata_local_connecteddiveces_platform = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Local\\ConnectedDevicesPlatform\\*",
        program: "Windows".parse().unwrap(),
        files_to_remove: vec![],
        category: "LastActivity".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![]
    };
    database.push(c_users_appdata_local_connecteddiveces_platform);
    let c_users_appdata_local_crash_dumps = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Local\\CrashDumps\\*",
        program: "Windows".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![]
    };
    database.push(c_users_appdata_local_crash_dumps);
    let c_users_downloads = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\Downloads\\*",
        program: "Windows".parse().unwrap(),
        files_to_remove: vec![],
        category: "Downloads".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![]
    };
    database.push(c_users_downloads);
    //</editor-fold>
    //<editor-fold desc="NVIDIA Corporation">
    let c_program_files_nvidia_corporation = CleanerData { 
        path: "C:\\Program Files\\NVIDIA Corporation".parse().unwrap(), 
        program: "NVIDIA Corporation".parse().unwrap(), 
        files_to_remove: vec!["license.txt".parse().unwrap()], 
        category: "Logs".parse().unwrap(), 
        remove_directories: false, 
        remove_files: false, 
        directories_to_remove: vec![], 
        remove_all_in_dir: false, 
        remove_directory_after_clean: false, 
        folders_to_remove: vec![]
    };
    database.push(c_program_files_nvidia_corporation);
    let c_program_files_nvidia_corporation_nvsmi = CleanerData { 
        path: "C:\\Program Files\\NVIDIA Corporation\\NVSMI".parse().unwrap(),
        program: "NVIDIA Corporation".parse().unwrap(), 
        files_to_remove: vec!["nvidia-smi.1.pdf".parse().unwrap()], 
        category: "Logs".parse().unwrap(), 
        remove_directories: false, 
        remove_files: false, 
        directories_to_remove: vec![], 
        remove_all_in_dir: false, 
        remove_directory_after_clean: false, 
        folders_to_remove: vec![] 
    };
    database.push(c_program_files_nvidia_corporation_nvsmi);
    //</editor-fold>
    //<editor-fold desc="Java">
    let java_1 = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\.jdks\\**",
        program: "Java".parse().unwrap(),
        files_to_remove: vec![
            "javafx-src.zip".parse().unwrap(),
            "src.zip".parse().unwrap()
        ],
        category: "Cache".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(java_1);
    let java_2 = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\.jdks\\**",
        program: "Java".parse().unwrap(),
        files_to_remove: vec![
            "NOTICE".parse().unwrap(),
            "COPYRIGHT".parse().unwrap(),
            "LICENSE".parse().unwrap(),
            "release".parse().unwrap(),
            "README".parse().unwrap(),
            "ADDITIONAL_LICENSE_INFO".parse().unwrap(),
            "ASSEMBLY_EXCEPTION".parse().unwrap(),
            "Welcome.html".parse().unwrap(),
            "THIRDPARTYLICENSEREADME-JAVAFX.txt".parse().unwrap(),
            "THIRDPARTYLICENSEREADME.txt".parse().unwrap(),
            "README.txt".parse().unwrap(),
            "DISCLAIMER".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(java_2);
    let java_5 = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\.jdks\\**",
        program: "Java".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![
            "sample".parse().unwrap(),
            "demo".parse().unwrap()
        ],
    };
    database.push(java_5);
    let java_2 = CleanerData {
        path: "C:\\Program Files\\Java\\**".parse().unwrap(),
        program: "Java".parse().unwrap(),
        files_to_remove: vec![
            "NOTICE".parse().unwrap(),
            "COPYRIGHT".parse().unwrap(),
            "LICENSE".parse().unwrap(),
            "release".parse().unwrap(),
            "README".parse().unwrap(),
            "ADDITIONAL_LICENSE_INFO".parse().unwrap(),
            "ASSEMBLY_EXCEPTION".parse().unwrap(),
            "Welcome.html".parse().unwrap(),
            "THIRDPARTYLICENSEREADME-JAVAFX.txt".parse().unwrap(),
            "THIRDPARTYLICENSEREADME.txt".parse().unwrap(),
            "README.txt".parse().unwrap(),
            "DISCLAIMER".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(java_2);
    let java_3 = CleanerData {
        path: "C:\\Program Files\\Eclipse Adoptium\\**".parse().unwrap(),
        program: "Java".parse().unwrap(),
        files_to_remove: vec![
            "NOTICE".parse().unwrap(),
            "COPYRIGHT".parse().unwrap(),
            "LICENSE".parse().unwrap(),
            "release".parse().unwrap(),
            "README".parse().unwrap(),
            "ADDITIONAL_LICENSE_INFO".parse().unwrap(),
            "ASSEMBLY_EXCEPTION".parse().unwrap(),
            "Welcome.html".parse().unwrap(),
            "THIRDPARTYLICENSEREADME-JAVAFX.txt".parse().unwrap(),
            "THIRDPARTYLICENSEREADME.txt".parse().unwrap(),
            "README.txt".parse().unwrap(),
            "DISCLAIMER".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(java_3);
    let java_4 = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\.loliland\\java",
        program: "Java".parse().unwrap(),
        files_to_remove: vec![
            "NOTICE".parse().unwrap(),
            "COPYRIGHT".parse().unwrap(),
            "LICENSE".parse().unwrap(),
            "release".parse().unwrap(),
            "README".parse().unwrap(),
            "ADDITIONAL_LICENSE_INFO".parse().unwrap(),
            "ASSEMBLY_EXCEPTION".parse().unwrap(),
            "Welcome.html".parse().unwrap(),
            "THIRDPARTYLICENSEREADME-JAVAFX.txt".parse().unwrap(),
            "THIRDPARTYLICENSEREADME.txt".parse().unwrap(),
            "README.txt".parse().unwrap(),
            "DISCLAIMER".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(java_4);
    //</editor-fold>
    //<editor-fold desc="4uKey for Android">
    let c_program_files_x86_tenorshare_4ukey_for_android_logs = CleanerData {
        path: "C:\\Program Files (x86)\\Tenorshare\\4uKey for Android\\Logs\\*".parse().unwrap(),
        program: "4uKey for Android".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_x86_tenorshare_4ukey_for_android_logs);
    let c_users_appdata_roaming_tsmonitor_4uker_for_android = CleanerData {
        path: "C:\\Users\\".to_owned() + username +"\\AppData\\Roaming\\TSMonitor\\4uKey for Android\\logs\\*",
        program: "4uKey for Android".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_tsmonitor_4uker_for_android);
    //</editor-fold>
    //<editor-fold desc="Postman">
    let c_users_appdata_roaming_postman_agent_logs = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\PostmanAgent\\logs\\*.log",
        program: "Postman".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_postman_agent_logs);
    let c_users_appdata_local_postman_agent = CleanerData {
        path: "C:\\Users\\".to_owned() + username +"\\AppData\\Local\\Postman-Agent\\*.log",
        program: "4uKey for Android".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_postman_agent);
    //</editor-fold>
    //<editor-fold desc="IDA Pro">
    let c_users_appdata_roaming_hex_rays_ida_pro = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\Hex-Rays\\IDA Pro\\*.lst",
        program: "IDA Pro".parse().unwrap(),
        files_to_remove: vec![],
        category: "Cache".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_hex_rays_ida_pro);

    //</editor-fold>
    //<editor-fold desc="Xamarin"">
    let c_users_appdata_local_xamarin_logs = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Local\\Xamarin\\Logs\\**\\*.log",
        program: "Xamarin".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_xamarin_logs);
    //</editor-fold>
    //<editor-fold desc="Windscribe"">
    let c_users_appdata_local_windscribe = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Local\\Windscribe\\Windscribe2\\*.txt",
        program: "Windscribe".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_windscribe);
    //</editor-fold>
    //<editor-fold desc="GitHub Desktop"">
    let c_users_appdata_roaming_github_desktop = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\GitHub Desktop\\*.log",
        program: "GitHub Desktop".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_github_desktop);
    let c_users_appdata_roaming_github_desktop_logs = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\GitHub Desktop\\logs\\*.log",
        program: "GitHub Desktop".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_github_desktop_logs);
    //</editor-fold>
    //<editor-fold desc="Panda Security"">
    let c_programdata_panda_security_pslogs = CleanerData {
        path: "C:\\ProgramData\\Panda Security\\PSLogs\\*.log".parse().unwrap(),
        program: "Panda Security".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_programdata_panda_security_pslogs);
    //</editor-fold>
    //<editor-fold desc="NetLimiter"">
    let c_programdata_panda_security_pslogs = CleanerData {
        path: "C:\\ProgramData\\Locktime\\NetLimiter\\**\\logs\\*.log".parse().unwrap(),
        program: "NetLimiter".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_programdata_panda_security_pslogs);
    //</editor-fold>
    //<editor-fold desc="MiniBin"">
    let c_program_files_x86_minibin = CleanerData {
        path: "C:\\Program Files (x86)\\MiniBin\\*.txt".parse().unwrap(),
        program: "MiniBin".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_x86_minibin);
    //</editor-fold>
    //<editor-fold desc="Brave Browser"">
    let c_program_files_brave_software_brave_browser_application = CleanerData {
        path: "C:\\Program Files\\BraveSoftware\\Brave-Browser\\Application\\*.log".parse().unwrap(),
        program: "Brave Browser".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_brave_software_brave_browser_application);
    let c_users_appdata_local_brave_software_brave_browser_user_data_default = CleanerData {
        path: "C:\\Users\\".to_owned() + username+ "\\AppData\\Local\\BraveSoftware\\Brave-Browser\\User Data\\Default",
        program: "Brave Browser".parse().unwrap(),
        files_to_remove: vec![
            "Favicons".parse().unwrap(),
            "Favicons-journal".parse().unwrap(),
            "History".parse().unwrap(),
            "History-journal".parse().unwrap(),
            "Visited Links".parse().unwrap()
        ],
        category: "Cache".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_brave_software_brave_browser_user_data_default);
    let c_users_appdata_local_brave_software_brave_browser_user_data_default_dawn_cache = CleanerData {
        path: "C:\\Users\\".to_owned() + username +"\\AppData\\Local\\BraveSoftware\\Brave-Browser\\User Data\\Default\\DawnCache",
        program: "Brave Browser".parse().unwrap(),
        files_to_remove: vec![
            "data_0".parse().unwrap(),
            "data_1".parse().unwrap(),
            "data_2".parse().unwrap(),
            "data_3".parse().unwrap(),
            "index".parse().unwrap()
        ],
        category: "Cache".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_brave_software_brave_browser_user_data_default_dawn_cache);
    let c_users_appdata_local_brave_software_brave_browser_user_data_default_gpu_cache = CleanerData {
        path: "C:\\Users\\".to_owned() + username +"\\AppData\\Local\\BraveSoftware\\Brave-Browser\\User Data\\Default\\GPUCache",
        program: "Brave Browser".parse().unwrap(),
        files_to_remove: vec![
            "data_0".parse().unwrap(),
            "data_1".parse().unwrap(),
            "data_2".parse().unwrap(),
            "data_3".parse().unwrap(),
            "index".parse().unwrap()
        ],
        category: "Cache".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_brave_software_brave_browser_user_data_default_gpu_cache);
    //</editor-fold>
    //<editor-fold desc="Mem Reduct"">
    let c_program_files_brave_software_brave_browser_application = CleanerData {
        path: "C:\\Program Files\\Mem Reduct".parse().unwrap(),
        program: "Mem Reduct".parse().unwrap(),
        files_to_remove: vec![
            "History.txt".parse().unwrap(),
            "License.txt".parse().unwrap(),
            "Readme.txt".parse().unwrap(),
            "memreduct.exe.sig".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_brave_software_brave_browser_application);
    //</editor-fold>
    //<editor-fold desc="qBittorrent"">
    let c_program_files_qbittorent = CleanerData {
        path: "C:\\Program Files\\qBittorrent\\*.pdb".parse().unwrap(),
        program: "qBittorrent".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_qbittorent);
    let c_program_files_qbittorent_logs = CleanerData {
        path: "C:\\Program Files\\qBittorrent\\logs\\*.log".parse().unwrap(),
        program: "qBittorrent".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_qbittorent_logs);
    //</editor-fold>
    //<editor-fold desc="CCleaner"">
    let c_program_files_ccleaner_logs = CleanerData {
        path: "C:\\Program Files\\CCleaner\\LOG\\*".parse().unwrap(),
        program: "CCleaner".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_ccleaner_logs);
    //</editor-fold>
    //<editor-fold desc="IObit Malware Fighter"">
    let c_program_files_ccleaner_logs = CleanerData {
        path: "C:\\ProgramData\\IObit\\IObit Malware Fighter\\*.log".parse().unwrap(),
        program: "IObit Malware Fighter".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_ccleaner_logs);
    let c_program_data_iobit_iobit_malware_finghter_homepage_advisor = CleanerData {
        path: "C:\\ProgramData\\IObit\\IObit Malware Fighter\\Homepage Advisor\\*.log".parse().unwrap(),
        program: "IObit Malware Fighter".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_data_iobit_iobit_malware_finghter_homepage_advisor);
    //</editor-fold>
    //<editor-fold desc="IObit Driver Booster"">
    let c_users_appdata_roaming_iobit_driver_booster_logs = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\IObit\\Driver Booster\\Logs\\*",
        program: "IObit Driver Booster".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_iobit_driver_booster_logs);
    let c_program_files_x86_iobit_driver_booster = CleanerData {
        path: "C:\\Program Files (x86)\\IObit\\Driver Booster\\*.log".parse().unwrap(),
        program: "IObit Driver Booster".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_x86_iobit_driver_booster);
    let c_program_files_x86_iobit_driver_booster_1 = CleanerData {
        path: "C:\\Program Files (x86)\\IObit\\Driver Booster\\*.txt".parse().unwrap(),
        program: "IObit Driver Booster".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_x86_iobit_driver_booster_1);
    //</editor-fold>
    //<editor-fold desc="Process Lasso"">
    let c_program_data_process_lasso_logs = CleanerData {
        path: "C:\\ProgramData\\ProcessLasso\\logs\\*".parse().unwrap(),
        program: "Process Lasso".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_data_process_lasso_logs);
    //</editor-fold>
    //<editor-fold desc="OBS Studio"">
    let c_program_files_obs_studio_bin_64bit = CleanerData {
        path: "C:\\Program Files\\obs-studio\\bin\\64bit\\*.log".parse().unwrap(),
        program: "OBS Studio".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_obs_studio_bin_64bit);
    let c_users_appdata_roaming_obs_studio_logs = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\obs-studio\\logs\\*txt",
        program: "OBS Studio".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_obs_studio_logs);
    //</editor-fold>
    //<editor-fold desc="Unity Hub"">
    let c_program_files_unity_hub = CleanerData {
        path: "C:\\Program Files\\Unity Hub\\*.html".parse().unwrap(),
        program: "Unity Hub".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_unity_hub);
    //</editor-fold>
    //<editor-fold desc="KeePass 2""">
    let c_program_files_keepass_password_safe_2 = CleanerData {
        path: "C:\\Program Files\\KeePass Password Safe 2\\*.txt".parse().unwrap(),
        program: "KeePass 2".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_keepass_password_safe_2);
    //</editor-fold>
    //<editor-fold desc="1Password""">
    let c_users_appdata_local_1password_logs_setup = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Local\\1Password\\logs\\setup\\*.log",
        program: "1Password".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_1password_logs_setup);
    //</editor-fold>
    //<editor-fold desc="LGHUB""">
    let c_program_files_lghub = CleanerData {
        path: "C:\\Program Files\\LGHUB".parse().unwrap(),
        program: "LGHUB".parse().unwrap(),
        files_to_remove: vec![
            "LICENSE".parse().unwrap(),
            "LICENSES.chromium.html".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_lghub);
    //</editor-fold>
    //<editor-fold desc="DeepL""">
    let c_users_appdata_local_deepl_se_logs = CleanerData {
        path: "C:\\Users\\{username}\\AppData\\Local\\DeepL_SE\\logs\\*".parse().unwrap(),
        program: "LGHUB".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_deepl_se_logs);
    let c_users_appdata_local_deepl_se_cache = CleanerData {
        path: "C:\\Users\\{username}\\AppData\\Local\\DeepL_SE\\cache\\*".parse().unwrap(),
        program: "LGHUB".parse().unwrap(),
        files_to_remove: vec![],
        category: "Cache".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_deepl_se_cache);
    //</editor-fold>
    //<editor-fold desc="Microsoft Lobe""">
    let c_users_appdata_roaming_lobe_logs = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\Lobe\\logs\\*",
        program: "Microsoft Lobe".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_lobe_logs);
    //</editor-fold>
    //<editor-fold desc="Tonfotos Telegram Connector""">
    let c_users_pictures_tonfotos_telegram_connector = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\Pictures\\Tonfotos Telegram Connector\\*",
        program: "Tonfotos Telegram Connector".parse().unwrap(),
        files_to_remove: vec![],
        category: "Images".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_pictures_tonfotos_telegram_connector);
    //</editor-fold>
    //<editor-fold desc="DotNet""">
    let c_program_files_x86_dotnet = CleanerData {
        path: "C:\\Program Files (x86)\\dotnet\\*.txt".parse().unwrap(),
        program: "DotNet".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_x86_dotnet);
    let c_users_dotnet_telemetry_storage_service = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\.dotnet\\TelemetryStorageService\\*",
        program: "DotNet".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_dotnet_telemetry_storage_service);
    //</editor-fold>
    //<editor-fold desc="MCCreator""">
    let c_users_mccreator_logs = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\.mcreator\\logs\\*.log",
        program: "MCCreator".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_mccreator_logs);
    //</editor-fold>
    //<editor-fold desc="7-Zip""">
    let c_program_files_7_zip = CleanerData {
        path: "C:\\Program Files\\7-Zip\\*.txt".parse().unwrap(),
        program: "7-Zip".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_7_zip);
    //</editor-fold>
    //<editor-fold desc="Tribler""">
    let c_users_appdata_roaming_tribler = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\.Tribler\\*.log",
        program: "Tribler".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_tribler);
    //</editor-fold>
    //<editor-fold desc="I2P""">
    let c_users_appdata_local_i2peasy_addressbook = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Local\\i2peasy\\addressbook",
        program: "I2P".parse().unwrap(),
        files_to_remove: vec![
            "log.txt".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_i2peasy_addressbook);
    let c_users_appdata_local_i2peasy = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Local\\i2peasy",
        program: "I2P".parse().unwrap(),
        files_to_remove: vec![
            "eventlog.txt".parse().unwrap(),
            "wrapper.log".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_i2peasy);
    let c_users_appdata_local_i2peasy_logs = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Local\\i2peasy\\logs\\*",
        program: "I2P".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_i2peasy_logs);
    let c_users_appdata_local_i2peasy_licenses = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Local\\i2peasy\\licenses\\*",
        program: "I2P".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_i2peasy_licenses);
    //</editor-fold>
    //<editor-fold desc="BoxedAppPacker""">
    let c_program_filex_x86_boxedapppacker = CleanerData {
        path: "C:\\Program Files (x86)\\BoxedAppPacker".parse().unwrap(),
        program: "BoxedAppPacker".parse().unwrap(),
        files_to_remove: vec![
            "changelog.txt".parse().unwrap(),
            "HomePage.url".parse().unwrap(),
            "purchase.url".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_filex_x86_boxedapppacker);
    //</editor-fold>
    //<editor-fold desc="Enigma Virtual Box""">
    let c_program_files_enigma_virtual_box = CleanerData {
        path: "C:\\Program Files (x86)\\Enigma Virtual Box".parse().unwrap(),
        program: "Enigma Virtual Box".parse().unwrap(),
        files_to_remove: vec![
            "help.chm".parse().unwrap(),
            "History.txt".parse().unwrap(),
            "License.txt".parse().unwrap(),
            "site.url".parse().unwrap(),
            "forum.url".parse().unwrap(),
            "support.url".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_enigma_virtual_box);
    //</editor-fold>
    //<editor-fold desc="GnuPG""">
    let c_program_files_gnupg = CleanerData {
        path: "C:\\Program Files (x86)\\GnuPG".parse().unwrap(),
        program: "GnuPG".parse().unwrap(),
        files_to_remove: vec![
            "README.txt".parse().unwrap(),
            "VERSION".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_gnupg);
    //</editor-fold>
    //<editor-fold desc="Gpg4win""">
    let c_program_files_enigma_x86_gpg4win = CleanerData {
        path: "C:\\Program Files (x86)\\Gpg4win".parse().unwrap(),
        program: "Gpg4win".parse().unwrap(),
        files_to_remove: vec![
            "VERSION".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_enigma_x86_gpg4win);
    //</editor-fold>
    //<editor-fold desc="Inno Setup 6""">
    let c_program_files_enigma_x86_inno_setup_6 = CleanerData {
        path: "C:\\Program Files (x86)\\Inno Setup 6".parse().unwrap(),
        program: "Inno Setup 6".parse().unwrap(),
        files_to_remove: vec![
            "whatsnew.htm".parse().unwrap(),
            "isfaq.url".parse().unwrap(),
            "license.txt".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_enigma_x86_inno_setup_6);
    let c_program_files_enigma_x86_inno_setup_6 = CleanerData {
        path: "C:\\Program Files (x86)\\Inno Setup 6\\Examples\\*.txt".parse().unwrap(),
        program: "Inno Setup 6".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_enigma_x86_inno_setup_6);
    //</editor-fold>
    //<editor-fold desc="VirtualBox""">
    let c_users_virtualbox_vms_logs = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\VirtualBox VMs\\**\\Logs\\*.log",
        program: "VirtualBox".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_virtualbox_vms_logs);
    let c_users_virtualbox_vms = CleanerData {
        path: "C:\\Program Files\\Oracle\\VirtualBox\\*.rtf".parse().unwrap(),
        program: "VirtualBox".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_virtualbox_vms);
    let c_users_virtualbox_vms_doc = CleanerData {
        path: "C:\\Program Files\\Oracle\\VirtualBox\\doc\\*".parse().unwrap(),
        program: "VirtualBox".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_virtualbox_vms_doc);
    //</editor-fold>
    //<editor-fold desc="Recaf""">
    let c_users_appdata_roaming_recaf = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\Recaf\\*.log",
        program: "Recaf".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_recaf);
    //</editor-fold>
    //<editor-fold desc="Process Hacker 2""">
    let c_program_files_process_hacker_2 = CleanerData {
        path: "C:\\Program Files\\Process Hacker 2".parse().unwrap(),
        program: "Process Hacker 2".parse().unwrap(),
        files_to_remove: vec![
            "CHANGELOG.txt".parse().unwrap(),
            "COPYRIGHT.txt".parse().unwrap(),
            "LICENSE.txt".parse().unwrap(),
            "README.txt".parse().unwrap(),
            "ProcessHacker.sig".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_process_hacker_2);
    //</editor-fold>
    //<editor-fold desc="Docker""">
    let c_programdata_dockerdesktop = CleanerData {
        path: "C:\\ProgramData\\DockerDesktop\\*.txt".parse().unwrap(),
        program: "Docker".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_programdata_dockerdesktop);
    let c_users_appdata_local_docker_logs = CleanerData {
        path: "C:\\Users\\".to_owned() + username +"\\AppData\\Local\\Docker\\log\\**\\*",
        program: "Docker".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_docker_logs);
    let c_users_appdata_local_docker = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Local\\Docker\\*.txt",
        program: "Docker".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_docker);
    //</editor-fold>
    //<editor-fold desc="HiAlgo Boost""">
    let c_programdata_dockerdesktop = CleanerData {
        path: "C:\\Program Files (x86)\\HiAlgo\\Plugins\\BOOST".parse().unwrap(),
        program: "HiAlgo Boost".parse().unwrap(),
        files_to_remove: vec![
            "hialgo_eula.txt".parse().unwrap(),
            "Update Boost.log".parse().unwrap(),
            "UpdateListing.txt".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_programdata_dockerdesktop);
    //</editor-fold>
    //<editor-fold desc="SoundWire Server""">
    let c_program_files_x86_soundwire_server = CleanerData {
        path: "C:\\Program Files (x86)\\SoundWire Server".parse().unwrap(),
        program: "SoundWire Server".parse().unwrap(),
        files_to_remove: vec![
            "license.txt".parse().unwrap(),
            "opus_license.txt".parse().unwrap(),
            "readerwriterqueue_license.txt".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_x86_soundwire_server);
    //</editor-fold>
    //<editor-fold desc="System Informer""">
    let c_program_files_systeminformer = CleanerData {
        path: "C:\\Program Files\\SystemInformer".parse().unwrap(),
        program: "SoundWire Server".parse().unwrap(),
        files_to_remove: vec![
            "LICENSE.txt".parse().unwrap(),
            "README.txt".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_systeminformer);
    //</editor-fold>
    //<editor-fold desc="Sandboxie Plus""">
    let c_program_files_sandboxie_plus = CleanerData {
        path: "C:\\Program Files\\Sandboxie-Plus".parse().unwrap(),
        program: "SoundWire Server".parse().unwrap(),
        files_to_remove: vec![
            "Manifest0.txt".parse().unwrap(),
            "Manifest1.txt".parse().unwrap(),
            "Manifest2.txt".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_sandboxie_plus);
    //</editor-fold>
    //<editor-fold desc="JetBrains""">
    let c_program_files_jetbrains_license = CleanerData {
        path: "C:\\Program Files\\JetBrains\\**\\license".parse().unwrap(),
        program: "JetBrains".parse().unwrap(),
        files_to_remove: vec![
            "javahelp_license.txt".parse().unwrap(),
            "javolution_license.txt".parse().unwrap(),
            "launcher-third-party-libraries.html".parse().unwrap(),
            "saxon-conditions.html".parse().unwrap(),
            "third-party-libraries.html".parse().unwrap(),
            "yourkit-license-redist.txt".parse().unwrap(),
            "remote-dev-server.html".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_jetbrains_license);
    let c_program_files_jetbrains = CleanerData {
        path: "C:\\Program Files\\JetBrains\\**".parse().unwrap(),
        program: "JetBrains".parse().unwrap(),
        files_to_remove: vec![
            "LICENSE.txt".parse().unwrap(),
            "NOTICE.txt".parse().unwrap()
        ],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_program_files_jetbrains);
    //</editor-fold>
    //<editor-fold desc="Discord""">
    let c_users_appdata_local_discord = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Local\\Discord\\*.log",
        program: "Discord".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_discord);
    //</editor-fold>
    //<editor-fold desc="Guilded""">
    let c_users_appdata_roaming_guilded = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\Guilded\\*.log",
        program: "Guilded".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_guilded);
    //</editor-fold>
    //<editor-fold desc="Element""">
    let c_users_appdata_local_element_desktop = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Local\\element-desktop\\*.log",
        program: "Element".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_local_element_desktop);
    //</editor-fold>
    //<editor-fold desc="Telegram""">
    let c_users_appdata_roaming_telefram_desktop_tdata = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\Telegram Desktop\\tdata",
        program: "Telegram".parse().unwrap(),
        files_to_remove: vec![
            "key_datas".parse().unwrap()
        ],
        category: "Accounts".parse().unwrap(),
        remove_directories: false,
        remove_files: false,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_telefram_desktop_tdata);
    let c_users_appdata_roaming_telefram_desktop = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\Telegram Desktop\\*.txt",
        program: "Telegram".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_telefram_desktop);
    let c_users_appdata_roaming_telefram_desktop_logs = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\Telegram Desktop\\*.log",
        program: "Telegram".parse().unwrap(),
        files_to_remove: vec![],
        category: "Logs".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_telefram_desktop_logs);
    let c_users_appdata_roaming_telefram_desktop_tdata_emoji_cache = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\Telegram Desktop\\tdata\\emoji\\*cache_*",
        program: "Telegram".parse().unwrap(),
        files_to_remove: vec![],
        category: "Cache".parse().unwrap(),
        remove_directories: false,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_telefram_desktop_tdata_emoji_cache);
    let c_users_appdata_roaming_telefram_desktop_tdata_user_data_cache = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\Telegram Desktop\\tdata\\user_data\\cache\\**\\*",
        program: "Telegram".parse().unwrap(),
        files_to_remove: vec![],
        category: "Cache".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_telefram_desktop_tdata_user_data_cache);
    let c_users_appdata_roaming_telefram_desktop_tdata_user_data_media_cache = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\Telegram Desktop\\tdata\\user_data\\media_cache\\**\\*",
        program: "Telegram".parse().unwrap(),
        files_to_remove: vec![],
        category: "Cache".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_telefram_desktop_tdata_user_data_media_cache);
    //</editor-fold>
    //<editor-fold desc="Signal""">
    let c_users_appdata_roaming_signal = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\Signal\\logs\\*",
        program: "Telegram".parse().unwrap(),
        files_to_remove: vec![ ],
        category: "Logs".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_signal);
    let c_users_appdata_roaming_signal_update_cache = CleanerData {
        path: "C:\\Users\\".to_owned() + username + "\\AppData\\Roaming\\Signal\\update-cache\\*",
        program: "Telegram".parse().unwrap(),
        files_to_remove: vec![ ],
        category: "Cache".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: false,
        folders_to_remove: vec![],
    };
    database.push(c_users_appdata_roaming_signal_update_cache);
    //</editor-fold>

    //<editor-fold desc="Images">

    //<editor-fold desc="ShareX">
    let sharex_1 = CleanerData { path: "C:\\Users\\".to_owned() + username + "\\Documents\\ShareX\\Screenshots\\**\\*.jpg", program: "ShareX".parse().unwrap(), files_to_remove: vec![], category: "Images".parse().unwrap(), remove_directories: false, remove_files: true, directories_to_remove: vec![], remove_all_in_dir: false, remove_directory_after_clean: false, folders_to_remove: vec![] };
    database.push(sharex_1);
    let sharex_2 = CleanerData { path: "C:\\Users\\".to_owned() + username + "\\Documents\\ShareX\\Screenshots\\**\\*.png", program: "ShareX".parse().unwrap(), files_to_remove: vec![], category: "Images".parse().unwrap(), remove_directories: false, remove_files: true, directories_to_remove: vec![], remove_all_in_dir: false, remove_directory_after_clean: false, folders_to_remove: vec![] };
    database.push(sharex_2);
    //</editor-fold>

    //</editor-fold>
    //<editor-fold desc="Cheats">

    //<editor-fold desc="Weave">
    let weave_1 = CleanerData {
        path: "C:\\Weave\\*".parse().unwrap(),
        program: "Weave".parse().unwrap(),
        files_to_remove: vec![],
        category: "Cheats".parse().unwrap(),
        remove_directories: true,
        remove_files: true,
        directories_to_remove: vec![],
        remove_all_in_dir: false,
        remove_directory_after_clean: true,
        folders_to_remove: vec![]
    };
    database.push(weave_1);
    //</editor-fold>

    //</editor-fold>


    for data in database.iter().clone() {
        if !options.contains(&&*data.category) {
            options.push(&*data.category);
        }
    }
    let validator = |a: &[ListOption<&&str>]| {
        if a.len() < 1 {
            return Ok(Validation::Invalid("No category is selected!".into()));
        }
        else {
            return Ok(Validation::Valid);
        }
    };

    let formatter: MultiOptionFormatter<'_, &str> = &|a| format!("{} selected categories", a.len());
    let ans = MultiSelect::new("Select the clearing categories:", options)
        .with_validator(validator)
        .with_formatter(formatter)
        .prompt();

    let mut bytes_cleared = 0;
    let mut removed_files = 0;
    let mut removed_directories = 0;
    let mut cleared_programs:Vec<Cleared> = vec![];


    match ans {
        Ok(ans) => {
            for data in database.iter().clone() {
                let mut working = false;
                if ans.contains(&&*data.category) {
                    let results: Result<Paths, PatternError> = glob(&*data.path);
                    match results {
                        Ok(results) => {
                            for result in results {
                                match result {
                                    Ok(result) => {
                                        let is_dir: bool = result.is_dir();
                                        let is_file: bool = result.is_file();
                                        let path: &str = result.as_path().to_str().unwrap();
                                        let name: Option<&str> = result.file_name().unwrap().to_str();
                                        let mut lenght = 0;
                                        match result.metadata() {
                                            Ok(res) => { lenght += res.len(); }
                                            Err(_) => {}
                                        }
                                        //println!("Found: {}", path);
                                        for file in &data.files_to_remove {
                                            let file_path = path.to_owned() + "\\" + &*file;
                                            match fs::remove_file(file_path) {
                                                Ok(_) => {
                                                    working = true;
                                                    removed_files += 1;
                                                    bytes_cleared += lenght;
                                                    //println!("Removed file: {}", name.unwrap());
                                                }
                                                Err(_) => {}
                                            }
                                        }
                                        for directory in &data.directories_to_remove {
                                            let file_path = path.to_owned() + "\\" + &*directory;
                                            match fs::remove_dir_all(file_path) {
                                                Ok(_) => {
                                                    working = true;
                                                    removed_directories += 1;
                                                    bytes_cleared += lenght;
                                                    //println!("Removed file: {}", name.unwrap());
                                                }
                                                Err(_) => {}
                                            }
                                        }

                                        for dir in &data.directories_to_remove {
                                            let dir_path = path.to_owned() + "\\" + &*dir;
                                            match fs::remove_dir(dir_path) {
                                                Ok(_) => {
                                                    removed_directories += 1;
                                                    bytes_cleared += lenght;
                                                    working = true;
                                                    //println!("Removed directory: {}", name.unwrap());
                                                }
                                                Err(_) => {}
                                            }
                                        }

                                        //println!("Found: {}", path);
                                        if data.remove_files && is_file {
                                            match fs::remove_file(path) {
                                                Ok(_) => {
                                                    removed_files += 1;
                                                    bytes_cleared += lenght;
                                                    working = true;
                                                    //println!("Removed file: {}", name.unwrap());
                                                }
                                                Err(_) => {}
                                            }
                                        }
                                        if data.remove_directories && is_dir {
                                            match fs::remove_dir_all(path) {
                                                Ok(_) => {
                                                    removed_directories += 1;
                                                    bytes_cleared += lenght;
                                                    working = true;
                                                    //println!("Removed directory: {}", name.unwrap());
                                                }
                                                Err(_) => {}
                                            }
                                        }
                                        if data.remove_all_in_dir {
                                            let results: Result<Paths, PatternError> = glob(&*(path.to_owned() + "\\*"));
                                            let mut files = 0;
                                            let mut dirs = 0;
                                            match results {
                                                Ok(results) => {
                                                    for result in results {
                                                        match result {
                                                            Ok(result) => {
                                                                if result.is_file() {
                                                                    files += 1;
                                                                }
                                                                if result.is_dir() {
                                                                    dirs += 1;
                                                                }
                                                            }
                                                            Err(_) => {}
                                                        }
                                                    }
                                                    match fs::remove_dir_all(path) {
                                                        Ok(_) => {
                                                            working = true;
                                                            bytes_cleared += lenght;
                                                            removed_files += files;
                                                            removed_directories += dirs;
                                                        }
                                                        Err(_) => {}
                                                    }
                                                }
                                                Err(_) => {}
                                            }

                                        }
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
                let mut program = data.program.clone();
                if working {
                    let data = Cleared { Program: program };
                    if !cleared_programs.contains(&data) {
                        cleared_programs.push(data);
                    }
                }
            }
        },
        Err(_) => println!("Can't work with these categories"),
    }
    println!("Cleared programms:");
    let table = Table::new(cleared_programs).to_string();
    println!("{}", table);
    println!("Removed: {}", get_file_size_string(bytes_cleared));
    println!("Removed files: {}", removed_files);
    println!("Removed directories: {}", removed_directories);
    let mut s=String::new();
    stdin().read_line(&mut s).expect("Did not enter a correct string");




}
