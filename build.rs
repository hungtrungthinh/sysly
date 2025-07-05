use std::env;
use std::fs::File;
use std::io::Write;
use chrono::Datelike;

fn main() {
    let developer = "Thinh Nguyen <hungtrungthinh@gmail.com>";
    let build_time = chrono::Utc::now().to_rfc3339();
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    
    // Project history and origin
    let project_name = "Sysly";
    let project_start = "2019-07-01";
    let project_origin = "Created as an experiment when switching to a new MacBook.";
    let project_inspired = "Inspired by htop. Written in Rust using sysinfo.";
    let current_year = chrono::Utc::now().year();
    let development_years = current_year - 2019;
    
    let mut f = File::create("src/build_info.rs").unwrap();
    write!(
        f,
        "pub const PROJECT_NAME: &str = \"{}\";\npub const DEVELOPER: &str = \"{}\";\npub const BUILD_TIME: &str = \"{}\";\npub const VERSION: &str = \"{}\";\npub const PROJECT_START: &str = \"{}\";\npub const PROJECT_ORIGIN: &str = \"{}\";\npub const PROJECT_INSPIRED: &str = \"{}\";\npub const DEVELOPMENT_YEARS: u32 = {};\n",
        project_name, developer, build_time, version, project_start, project_origin, project_inspired, development_years
    ).unwrap();
}
