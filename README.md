# kittypaws
A tool for simulating destructive behavior of production infrastructure

![Alt Text](https://media.giphy.com/media/vFKqnCdLPNOKc/giphy.gif)

WIP mode. Yet works for a few months in our company (doesn't mean it's stable).

## Usage

Paws looks for config at `./paws.yml` path by default, but path to config can be specified after run command.

Using cargo:
```bash
cargo run -- run config.yml
```

Using distributive:
```bash
paws run config.yml
```

## Plugins

### Plugin management

Plugins are stored at `${PAWS_HOME}/plugins/`, which is `~/.kittypaws/plugins/` by default.

#### Install plugin

Install `subatiq/kittypaws-deathloop` plugin from github using `master` branch and save it by the name of `deathloop`:

```bash
paws install subatiq/kittypaws-deathloop master deathloop
```

Install `subatiq/kittypaws-deathloop` plugin from github using `master` branch and save it by the default name of `kittypaws-deathloop`:

```bash
paws install subatiq/kittypaws-deathloop master
```

#### Uninstall plugin

```bash
paws uninstall plugin-name
```

Use plugin name by which it was saved.

#### List installed plugins

```bash
paws list
```

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

### Known plugins

#### Dropper
Drops connection to a certain IP address. Works with Ubuntu-based docker containers\
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
- name: plugin01
  config01: yes
  config02: 42
  ...

- name: plugin01:
  config03: yes
  config02: 44
  ...
```
### Startup configuration

Plugins can start executing their tasks immediately, or after some time. You can configure them to wait for their interval first or wait for a specific delay and then continue to work normally.

#### Instant (hot) start

Config example:
```yaml
- name: <plugin_name>:
  ...
  startup: hot
```

#### Wait for the interval first (cold start)

Intervals configuration is described below.

Config example:
```yaml
- name: <plugin_name>:
  ...
  startup: cold  # works by default
  frequency:
    max: PT1M
    min: PT30S
```

Here first plugin run will be executed after random interval.


#### Delayed start

Config example:
```yaml
- name: <plugin_name>:
  ...
  startup: PT5S
```

With this config plugin will start after waiting 5 seconds, then it will only wait for random intervals between runs.

### Interval configuration

Time durations in config comply with ISO 8601.
Plugins can run in different intervals or once. To let kitty know how often you want them to run add this to plugin config:

#### Random intervals

Config example:
```yaml
- name: <plugin_name>:
  ...
  frequency:
    min: PT30S
    max: PT1M
```

#### Fixed intervals

Config example:
```yaml
- name: <plugin_name>:
  ...
  frequency: PT1M
```

#### Run once

Used as default

Config example:
```yaml
- name: <plugin_name>:
  ...
  frequency: once # or do not put it in config at all, it's default
```

---

Go get these bugs, Tiger!

![](https://cdn.discordapp.com/attachments/694259726619246674/1065994810210652180/image.png)
