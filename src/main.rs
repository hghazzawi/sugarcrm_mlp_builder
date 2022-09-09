use std::fs;
use clap::{App, load_yaml};
use chrono::prelude::*;
use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use zip::result::ZipError;
use zip::write::FileOptions;
use std::fs::File;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

#[macro_use]
extern crate dotenv_codegen;

fn main() {

    const METHOD_STORED: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Stored);
 
    // CLI argument setup
    let yaml = load_yaml!("resources/cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Capture Package Arguments
    let package_name = matches.value_of("name").unwrap();
    let version = matches.value_of("version").unwrap_or(dotenv!("DEFAULT_VERSION"));
    let files = matches.value_of("files").unwrap();
    let sugar_instance = matches.value_of("instance").unwrap();
    
    // Initialize folder and file structure
    let web_root=format!("{}{}{}",dotenv!("WEB_ROOT"),sugar_instance,"/");
    let base_path="src/";
    let mut copy_string=format!("");
    
    // Get list of files from argument
    let file_list_content=fs::read_to_string(files);
    let file_list_unwrapped=file_list_content.unwrap();
    let file_list = file_list_unwrapped.lines();
    
    // Initialize Package Manifest
    let manifest_template=include_bytes!("resources/manifest.txt");
    let manifest_content=String::from_utf8_lossy(manifest_template);
    let mut new_manifest_content=manifest_content.replace("__author__",dotenv!("AUTHOR"));
    new_manifest_content=new_manifest_content.replace("__version__",version);
    new_manifest_content=new_manifest_content.replace("__name__",package_name);
    new_manifest_content=new_manifest_content.replace("__description__",package_name);
    new_manifest_content=new_manifest_content.replace("__package_id__",&no_space(package_name.to_string()));
    
    let now = Utc::now();
    let res = now.format("%Y-%m-%d");
    let formatted_date=format!("{}", res);
    new_manifest_content=new_manifest_content.replace("__published_date__",&formatted_date);


    // Loop through file list and copy into releases folder
    for file in file_list {

        let file_name=format!("{}{}",base_path,file);
        
        let path=std::path::Path::new(&file_name);

        let prefix=path.parent().unwrap();

        fs::create_dir_all(format!("{}{}{}{}","releases/",package_name,"/",prefix.display())).unwrap();

        let from_copy=format!("{}{}",web_root,file);

        let to_copy=format!("{}{}{}{}{}","releases/",package_name,"/",base_path,file);

        let from_manifest_copy=format!("{}{}",base_path,file);

        let copy_entry=format!("array(\n\t\t'from' => '<basepath>/{}',\n\t\t'to' => '{}'\n\t\t),\n\n\t\t",from_manifest_copy,file);

        copy_string.push_str(&copy_entry);

        fs::copy(from_copy,to_copy).unwrap();
    }

    // Update manifest with copy array
    new_manifest_content=new_manifest_content.replace("__copy__",copy_string.as_str());

    // create manifest file
    let manifest_path=format!("{}{}{}","releases/",package_name,"/manifest.php");
    fs::write(manifest_path,new_manifest_content).unwrap();

    create_release_package(&format!("{}{}","releases/",package_name),&format!("{}{}{}","releases/",package_name,".zip"),METHOD_STORED.unwrap()).unwrap();

    fs::remove_dir_all(&format!("{}{}","releases/",package_name)).unwrap();

}


fn no_space(x : String) -> String{
    x.replace(" ", "")
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {:?} as {:?} ...", path, name);
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {:?} as {:?} ...", path, name);
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
    
}

fn create_release_package(
    src_dir: &str,
    dst_file: &str,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}