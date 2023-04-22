# kittypaws
A tool for simulating destructive behavior of production infrastructure

![Alt Text](https://media.giphy.com/media/vFKqnCdLPNOKc/giphy.gif)

WIP mode. Yet works for a few months in our company (doesn't mean it's stable).

## Requirements:
You need to install the following packages:

1. Rust from [official website](https://www.rust-lang.org/tools/install)
- Don't forget to add `Cargo` to `PATH` using:
    ```bash
    source ~/.cargo/env
    ```
2. `build-essential` for C/C++ compiler support:
    ```bash
    apt install build-essential
    ```
3. `python3-dev` for Rust's Python bindings:

    ```bash
    apt install python3-dev
    ```


## Usage

1. Create the following directory (in `$HOME`):
    `/.kittypaws/plugins/`
2. In `plugins` folder, create folder(s) with the name(s) of the plugin(s) you wanna use.

    * For example if you want to use `timeburglar`:

        `~/.kittypaws/plugins/timeburglar`
3. Copy the plugin's `main.py` or `run.sh` to the plugin name folder from the previous step

4. Create a file `config.yml` in the root of this repo's folder 
    * The content depends on the plugins used
    * `Timeburglar` config file example:
        ```bash
        plugins:
        # Disrupting host time
        - timeburglar:
            shift: 1
            startup: hot
            frequency: fixed
            interval: PT60S
        ```
5. Run using `cargo`:
    ```bash
    cargo run config.yml
    ```

    or using distributive:
    ```bash
    paws config.yml
    ```

## Plugins

### How to write a new plugin

Plugins are stored in `~/.kittypaws/plugins/{plugin_name}` with `main.py` or `run.sh` file inside

#### Python files

Inside the plugin name folder, `main.py` with `run` function:

```python
def run(config: Dict[str, str]) -> None:
   pass
```

#### Bash

Inside the the plugin name folder, `run.sh` should contain:

```bash
config_field1=${config_field1:-default_value}

echo config_field1
```

Kittypaws will load it if the plugin name is in the config matches the plugin folder name and will run it with the specified frequency.

### Known plugins


#### Dropper
Drops connection to a certain IP address\
https://github.com/subatiq/kittypaws-dropper

#### Deathloop
Simply restarts target container\
https://github.com/subatiq/kittypaws-deathloop

#### Time burglar
Breaks time sync on the host\
https://github.com/subatiq/kittypaws-timeburglar


### Configuration structure

```yaml
plugins:
- plugin01:
    config01: yes
    config02: 42
    ...
    
- plugin01:
    config03: yes
    config02: 44
    ...
```
### Startup configuration 

Plugins can start executing their tasks immediately, or after some time. You can configure them to wait for their interval first or wait for a specific delay and then continue to work normally.

#### Instant (hot) start

Config example: 
```yaml
- <plugin_name>: 
    ...
    startup: hot
```

#### Wait for the interval first (cold start)

Intervals configuration is described below.

Config example: 
```yaml
- <plugin_name>: 
    ...
    startup: cold  # works by default
    frequency: random
    max_interval: PT1M
    min_interval: PT30S
```

Here first plugin run will be executed after random interval.


#### Delayed start

Config example: 
```yaml
- <plugin_name>: 
    ...
    startup: PT5S
    frequency: random
    max_interval: PT1M
    min_interval: PT30S
```

With this config plugin will start after waiting 5 seconds, then it will only wait for random intervals between runs.

### Interval configuration

Time durations in config comply with ISO 8601.
Plugins can run in different intervals or once. To let kitty know how often you want them to run add this to plugin config:

#### Random intervals

Config example: 
```yaml
- <plugin_name>: 
    ...
    frequency: random
    max_interval: PT1M
    min_interval: PT30S
```

#### Fixed intervals

Config example: 
```yaml
- <plugin_name>: 
    ...
    frequency: fixed
    interval: PT1M
```

#### Run once

Used as default

Config example: 
```yaml
- <plugin_name>: 
    ...
    frequency: once # or do not put it in config at all, it's default
```


---

Go get these bugs, Tiger!

![](https://cdn.discordapp.com/attachments/694259726619246674/1065994810210652180/image.png)
