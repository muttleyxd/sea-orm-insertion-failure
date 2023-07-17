#!/usr/bin/env bash
set -euxo pipefail

docker-compose -f docker-db.yml up -d
