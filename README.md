# kittypaws
A tool for simulating destructive behavior of production infrastructure

![Alt Text](https://media.giphy.com/media/vFKqnCdLPNOKc/giphy.gif)

*HEAVY WIP MODE*

*DO NOT GO PASS THAT TEXT*


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

Plugins are stored in `~/.kittypaws/plugins/` each in a folder named after plugin. Inside the folder there should be `main.py` with the `run` function:

```python
def run(config: Dict[str, str]) -> None:
   pass
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

