# kittypaws
A tool for simulating destructive behavior of production infrastructure

![Alt Text](https://media.giphy.com/media/vFKqnCdLPNOKc/giphy.gif)

WIP mode. Yet works for a few months already in our company (doesn't mean it's stable).

## Usage

Using cargo:
```bash
cargo run config.yml
```

Using distributive:
```bash
paws config.yml
```

## Plugins

### How to write a new plugin

Plugins are stored in `~/.kittypaws/plugins/` each in a folder named after plugin. 

#### Python

Inside the folder there should be `main.py` with the `run` function:

```python
def run(config: Dict[str, str]) -> None:
   pass
```

#### Bash

Inside the folder there should be `run.sh`:

```bash
config_field1=${config_field1:-default_value}

echo config_field1
```

Kittypaws will load it if plugin name is in the config and run with specified frequency.

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
