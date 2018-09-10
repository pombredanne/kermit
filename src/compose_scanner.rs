use failure::Error;
use std::fs;
use walkdir::WalkDir;
use serde_json;

pub struct Scanner {}

#[derive(Serialize, Deserialize)]
pub struct ScanResult {
    pub path: String,
}

impl ScanResult {
    fn new(path: &str) -> ScanResult {
        ScanResult {
            path: String::from(path),
        }
    }
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {}
    }
}

impl Scanner {
    pub fn scan(&self, base_path: &str) -> Result<Vec<ScanResult>, Error> {
        let mut return_vec = Vec::new();
        for entry in WalkDir::new(base_path) {
            match entry {
                Ok(ref entry) => {
                    let path = entry.path().to_str().expect("path with no name");
                    if !check_compose_directory(path) {
                        continue;
                    }
                    return_vec.push(ScanResult::new(path));
                }
                Err(_e) => continue,
            }
        }
        Ok(return_vec)
    }

    pub fn to_file(&self, path: &str, out_file: &str) -> Result<(), Error> {
        let scan_result = self.scan(path);
        match scan_result {
            Ok(res) => {
                let json = serde_json::to_string(&res).unwrap();
                println!("{}", json);
            }
            Err(e) => return Err(format_err!("failed to serialize: {}", e)),
        }
        Ok(())
    }
}

fn check_compose_directory(path: &str) -> bool {
    let meta = fs::metadata(path);
    if !meta.is_ok() {
        return false;
    }
    // unwrap is ok because we checked above
    if !meta.unwrap().is_dir() {
        return false;
    }
    let mut path = String::from(path);
    path.push_str("/docker-compose.yml");
    fs::metadata(path).is_ok()
}
