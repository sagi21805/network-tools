#! /bin/sh
sudo nmcli device wifi connect $1 password $2 > /dev/null 2>&1