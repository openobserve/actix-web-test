#!/bin/bash

docker run -tdi --name filebeat \
  -v "/Users/yanghengfei/code/docker/data/nginx/log:/var/log/nginx" \
  -v "/Users/yanghengfei/code/docker/data/filebeat/config/filebeat.yml:/usr/share/filebeat/filebeat.yml" \
  docker.elastic.co/beats/filebeat:8.7.0