#!/bin/bash

DOCKER_IP=`ip addr | egrep 'inet.*docker' | awk '{ print $2 }' | cut -d '/' -f1`
docker run -it --rm --entrypoint=mosquitto_sub eclipse-mosquitto -h $DOCKER_IP -t status -u admin
