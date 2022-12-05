import time
from subprocess import check_output

def run(config: dict[str, str]) -> None:
    print("Dropper plugin running")
    print("Config: ", config)

    target = config.get('target')
    ip = config.get('ip')
    unavailable_seconds = int(config.get('unavailable_seconds', 10))
    available_seconds = int(config.get('available_seconds', 10))
    print('Installing iptables...')
    output = check_output(f'docker exec {target} apt-get install iptables -y', shell=True)
    print(output)

    while True:
        output = check_output(f"docker exec {target} bash -c 'iptables -C OUTPUT -d {ip} -j DROP || iptables -I OUTPUT -d {ip} -j DROP'", shell=True)
        print(f'--- {ip} is unavailable now for {target}. Switching in {unavailable_seconds} sec')
        time.sleep(unavailable_seconds)
        output = check_output(f"docker exec {target} bash -c 'iptables -D OUTPUT -d {ip} -j DROP'", shell=True)
        print(f'+++ {ip} is available now for {target}. Switching in {available_seconds} sec')
        time.sleep(available_seconds)
