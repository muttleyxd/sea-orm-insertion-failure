#!/usr/bin/env bash
set -euxo pipefail

docker-compose -f docker-db.yml rm -fsv || true
docker volume rm sea-orm-insertion-failure_mariadb_data
