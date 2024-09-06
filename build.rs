use std::env;
use std::path::PathBuf;
use std::fs;
use std::path::Path;

fn main() {
    // 定義資源資料夾和輸出資料夾的路徑
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("resources");
    let source = Path::new("resources");

    // 複製整個 resources 資料夾到輸出目錄
    if source.exists() {
        fs::create_dir_all(&dest).unwrap();
        for entry in fs::read_dir(source).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name();
            let dest_file = dest.join(file_name);
            fs::copy(entry.path(), dest_file).unwrap();
        }
    }
    
    let target = env::var("TARGET").unwrap();
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();
        if target.contains("msvc") {
            lib_dir.push("msvc");
            dll_dir.push("msvc");
        } else {
            lib_dir.push("gnu-mingw");
            dll_dir.push("gnu-mingw");
        }
        lib_dir.push("lib");
        dll_dir.push("dll");
        if target.contains("x86_64") {
            lib_dir.push("64");
            dll_dir.push("64");
        } else {
            lib_dir.push("32");
            dll_dir.push("32");
        }

        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir") {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path()).expect("Can't copy from DLL dir");
                }
            }
        }
    }
    
    if target.contains("wasm") {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let dest = Path::new(&out_dir).join("index.html");

        // 複製 HTML 檔案到輸出目錄
        fs::copy("static/index.html", dest).unwrap();
    }
}