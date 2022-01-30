#!/bin/bash

docker run -it --rm -p 1883:1883 -v `pwd`/mosquitto_config/mosquitto.conf:/mosquitto/config/mosquitto.conf -v `pwd`/mosquitto_config/acl_file.conf:/mosquitto/config/acl_file.conf eclipse-mosquitto
