#! /bin/sh

ips=$(sudo arp-scan -l | grep -Eo '([0-9]*\.){3}[0-9]{1,3}')

for ip in $ips; do
    nmap -sC $ip
done