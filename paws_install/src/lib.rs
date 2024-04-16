use std::{env, fs::File, io::Write, path::{PathBuf, Components, Path, Component}, ffi::OsStr};

use zip::ZipArchive;

fn get_kittypaws_home() -> PathBuf {
    PathBuf::from(env::var("PAWS_HOME").unwrap_or(unwrap_home_path("~/.kittypaws")))
}

fn get_plugin_path(plugin_name: &str) -> PathBuf {
    get_kittypaws_home().join("plugins").join(plugin_name)
}

pub fn remove_plugin(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_plugin_path(&name);
    if !path.exists() {
        println!("Plugin {} isn't installed", name);
        return Ok(());
    }

    println!("Removing {}...", name);

    std::fs::remove_dir_all(get_plugin_path(&name))?;

    println!("Plugin {} removed!", name);
    Ok(())
}

pub fn list_plugins() -> Result<(), Box<dyn std::error::Error>> {
    let path: PathBuf = get_kittypaws_home().join("plugins");

    println!("Installed plugins:");

    for path in path.read_dir().unwrap() {
        let path = path.unwrap().path();
        if !path.is_dir() {
            continue;
        }

        println!("- {}", path.file_name().unwrap().to_str().unwrap());
    }

    Ok(())
}

pub fn install_from_github(
    repo_spec: &str,
    branch: &str,
    save_as: Option<String>,
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

    let destination = get_kittypaws_home().join("plugins");

    println!("Unpacking ZIP archive...");

    let mut zip = ZipArchive::new(temp_file)?;
    let plugin_save_name = save_as.unwrap_or(repo_spec.split_once('/').unwrap().1.to_string());

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;

        let file_name = file.mangled_name();
        let components: Components = file_name.components();
        let mut components: Vec<_> = components.into_iter().collect();
        components.remove(0);
        components.insert(0, Component::Normal(&OsStr::new(&plugin_save_name)));

        let file_path = PathBuf::from_iter(components);

        let outpath = destination.join(file_path);

        if file.is_dir() {
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
    println!("Installed {} as {}", repo_spec, plugin_save_name);

    Ok(())
}

fn unwrap_home_path(path: &str) -> String {
    if path.starts_with('~') {
        match std::env::var("HOME") {
            Ok(home) => path.replace('~', &home).to_string(),
            Err(_) => {
                println!("Could not find home directory");
                path.to_string()
            }
        }
    } else {
        path.to_string()
    }
}

#[test]
fn test_install() {
    //let result = install_from_github("subatiq/kittypaws-deathloop", "master");
    //println!("{:?}", result);
    //assert!(result.is_ok());
}
