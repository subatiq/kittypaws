use rand::Rng;
use tokio::runtime::Runtime;
use iso8601_duration::Duration;
use pluginterface::{PlugInterface, PlugConfig};
use docker_api::{Docker, Result, Container};

#[no_mangle]
pub extern "Rust" fn new_service() -> Box<dyn PlugInterface> {
    Box::new(ContainerRestartPlugin::new())
}


#[cfg(unix)] pub fn new_docker() -> Result<Docker> {
    Ok(Docker::unix("/var/run/docker.sock"))
}


pub struct ContainerRestartPlugin {
    name: String,
}

impl ContainerRestartPlugin {
    fn new() -> ContainerRestartPlugin {
        let name = "deathloop";
        println!("[{}] Plugin started", name);
        ContainerRestartPlugin { name: name.to_string() }
    }
}

fn get_runtime() -> Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build().expect("Failed to create runtime")
}


fn restart_container(container: Container) -> Result<()> {
    let rt = get_runtime();

    rt.block_on(async move {
        container.restart(None).await.expect("Failed to restart container");
    });
    Ok(())
}

fn get_container_id(docker: &Docker, name: &str) -> Option<String> {
    use docker_api::opts::ContainerListOpts;
    let opts = ContainerListOpts::builder().all(true).build();
    let rt = get_runtime();
    

    let containers = rt.block_on(async move {
        return docker.containers().list(&opts).await.expect("Failed to list containers");
    });

    for container in containers {
        let names = container.names.unwrap();
        let id = container.id.unwrap();
        if names.contains(&format!("/{}", name)) {
            return Some(id);
        }
    }

    None
}


enum Frequency {
    Fixed,
    Random,
    Once
}

impl PlugInterface for ContainerRestartPlugin {
    fn run(&self, config: &PlugConfig) {
        loop {
            println!("[{}] Running with config: {:?}", self.name, config);
            let target = config.get("target").expect("No target specified");
            let frequency = match config.get("frequency") {
                Some(f) => match f.as_str() {
                    "fixed" => Frequency::Fixed,
                    "random" => Frequency::Random,
                    "once" => Frequency::Once,
                    _ => panic!("Invalid frequency")
                },
                None => Frequency::Fixed
            };

            let interval = Duration::parse(config.get("interval")
                .or(Some(&"PT10S".to_string())).unwrap())
                .unwrap().to_std().as_secs();

            let min_interval = Duration::parse(config.get("min_interval")
                .or(Some(&"PT5S".to_string())).unwrap())
                .unwrap().to_std().as_secs();

            let docker = new_docker().expect("Failed to connect to docker");
            let id = get_container_id(&docker, target).expect("Failed to find container");

            let container = docker.containers().get(id);
            println!("[{}] Restarting container {}", self.name, container.id());
            restart_container(container).expect("Failed to restart container");

            match frequency {
                Frequency::Fixed => std::thread::sleep(std::time::Duration::from_secs(interval)),
                Frequency::Random => {
                    let sleep_time = rand::thread_rng().gen_range(min_interval..interval);
                    println!("[{}] Done! Sleeping for {} seconds", self.name, sleep_time);
                    std::thread::sleep(std::time::Duration::from_secs(sleep_time));
                },
                Frequency::Once => break
            }
        }
    }
}

impl Drop for ContainerRestartPlugin {
    fn drop(&mut self) {
        println!("[{}] Plugin finished", self.name);
    }


}
