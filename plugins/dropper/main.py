import time
from subprocess import check_output

def run(config: dict[str, str]) -> None:
    print("Dropper plugin running")
    print("Config: ", config)

    target = config.get('target')
    url = config.get('url')
    output = check_output(f'docker exec {target} apt-get install iptables -y', shell=True)
    print(output)
    output = check_output(f'docker exec {target} iptables -I OUTPUT -d {url} -j DROP', shell=True)
    print(output)
    time.sleep(10)
    output = check_output(f'docker exec {target} iptables -D OUTPUT -d {url} -j DROP', shell=True)
    print(output)
