#!/bin/bash
vagrant ssh droplet -c "docker swarm init --advertise-addr \$(hostname -I | cut -d\" \" -f1)"
allips=$(vagrant ssh droplet -c "hostname -I")
ip=$(echo $allips | cut -d" " -f1)
ip=$(echo $ip | grep -oE '\b([0-9]{1,3}\.){3}[0-9]{1,3}\b')
ip=$ip:2377

token=$(vagrant ssh droplet -c 'docker swarm join-token manager -q')
token=$(echo "$token" | tr -d '\r')

echo "Token: $token"
echo "IP: $ip"

vagrant ssh worker1 -c "docker swarm join --token $token $ip"
vagrant ssh worker2 -c "docker swarm join --token $token $ip"