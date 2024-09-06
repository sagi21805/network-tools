#! /bin/bash
ip_regex='([0-9]{1,3}\.){3}[0-9]{1,3}'
sudo arp-scan -l | grep -Eo $ip_regex