use std::{fs::File, io::Write, path::PathBuf, str::FromStr};

use zip::ZipArchive;

pub fn install_from_github(
    repo_spec: &str,
    branch: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let uri = format!("https://github.com/{}/archive/{}.zip", repo_spec, branch);

    println!("Getting repo at {}...", uri);
    let response = minreq::get(uri).send()?;
    let temp_file_path = "./.tmp.zip";

    std::fs::remove_file(temp_file_path).ok();

    let content = response.as_bytes();

    let mut temp_file = std::fs::File::create(temp_file_path)?;
    temp_file.write_all(content)?;
    let temp_file = std::fs::File::open(temp_file_path)?;

    let destination = unwrap_home_path("~/.kittypaws/plugins");

    println!("Unpacking ZIP archive...");
    let mut zip = ZipArchive::new(temp_file)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        // TODO: Strip prefix and replace it by normalized plugin name
        // https://stackoverflow.com/questions/68237045/how-do-i-change-a-paths-first-ancestor-in-rust
        let outpath = destination.join(file.mangled_name());

        if (&*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    std::fs::remove_file(temp_file_path).ok();

    Ok(())
}

fn unwrap_home_path(path: &str) -> PathBuf {
    if path.starts_with('~') {
        match std::env::var("HOME") {
            Ok(home) => PathBuf::from(&path.replace('~', &home)),
            Err(_) => {
                println!("Could not find home directory");
                PathBuf::from(path)
            }
        }
    } else {
        PathBuf::from(path)
    }
}

#[test]
fn test_install() {
    let result = install_from_github("subatiq/kittypaws-deathloop", "master");
    println!("{:?}", result);
    assert!(result.is_ok());
}
