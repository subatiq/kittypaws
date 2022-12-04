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

### Deathloop

Restart target container at certain intervals

Config example: 
```yaml
deathloop: 
  target: container_name
  frequency: random
  interval: PT1M
  min_interval: PT30S
```

`frequency` can be 
- `random` - waits for random between `interval` and `min_interval`, 
- `fixed` - waits for `interval`, 
- `once` - just quits after one restart. 

Time durations comply with ISO 8601.

### Dropper

Drop connection to certain IP addresses

Config example: 
```yaml
dropper: 
  target: container_name
  ip: X.X.X.X
  available_seconds: 15
  unavailable_seconds: 10
```

