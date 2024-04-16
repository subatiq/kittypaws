use std::{
    env,
    ffi::OsStr,
    fs::File,
    io::Write,
    path::{Component, Components, PathBuf},
};

use zip::ZipArchive;

fn get_kittypaws_home() -> PathBuf {
    PathBuf::from(env::var("PAWS_HOME").unwrap_or(unwrap_home_path("~/.kittypaws")))
}

fn get_plugins_path() -> PathBuf {
    get_kittypaws_home().join("plugins")
}

fn get_plugin_path(plugin_name: &str) -> PathBuf {
    get_plugins_path().join(plugin_name)
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

fn get_all_plugins() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let path: PathBuf = get_kittypaws_home().join("plugins");
    if !path.exists() {
        return Ok(vec![]);
    }
    let mut plugins: Vec<String> = vec![];

    println!("{:?}", path);
    for path in path.read_dir().unwrap() {
        let path = path?.path();

        if !path.is_dir() {
            continue;
        }

        plugins.push(path.file_name().unwrap().to_str().unwrap().to_string());
    }

    Ok(plugins)
}

pub fn list_plugins() -> Result<(), Box<dyn std::error::Error>> {
    println!("Installed plugins:");

    for plugin in get_all_plugins()?.iter() {
        println!("- {}", plugin);
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

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{get_all_plugins, install_from_github, remove_plugin, get_plugins_path};

    const TEST_PLUGIN: &str = "test_plugin";

    struct TestCleanup;

    impl Drop for TestCleanup {
        fn drop(&mut self) {
            std::fs::remove_dir_all("./.tmp").ok();
        }
    }

    fn setup_test() {
        let _ = TestCleanup;
        env::set_var("PAWS_HOME", "./.tmp");
    }

    #[test]
    fn test_list_no_plugins_folder() {
        setup_test();

        let listed_plugins = get_all_plugins().unwrap();
        assert!(listed_plugins.is_empty());
    }

    #[test]
    fn test_list_empty_plugins_folder() {
        setup_test();
        std::fs::create_dir_all(get_plugins_path()).unwrap();

        let listed_plugins = get_all_plugins().unwrap();
        assert!(listed_plugins.is_empty());
    }

    #[test]
    fn test_install() {
        setup_test();


        install_from_github(
            "subatiq/kittypaws-deathloop",
            "master",
            Some(TEST_PLUGIN.to_string()),
        )
        .unwrap();

        let listed_plugins = get_all_plugins().unwrap();
        dbg!(&listed_plugins);
        assert!(listed_plugins.contains(&TEST_PLUGIN.to_string()));
    }

    #[test]
    fn test_uninstall() {
        setup_test();
        std::fs::create_dir_all(get_plugins_path().join(TEST_PLUGIN)).unwrap();

        remove_plugin(TEST_PLUGIN.to_string()).unwrap();

        let listed_plugins = get_all_plugins().unwrap();
        assert!(!listed_plugins.contains(&TEST_PLUGIN.to_string()));
    }
}
