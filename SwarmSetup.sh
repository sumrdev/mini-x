#!/bin/bash
ip=$(vagrant ssh droplet -c 'hostname -I | cut -d" " -f1')
vagrant ssh droplet -c "docker swarm init --advertise-addr $ip"
token=vagrant ssh droplet -c 'docker swarm join-token manager -q'

echo "Token: $token"
echo "IP: $ip"

#vagrant ssh droplet2 -c "docker swarm join --token $token $ip:2377"
#vagrant ssh droplet3 -c "docker swarm join --token $token $ip:2377"