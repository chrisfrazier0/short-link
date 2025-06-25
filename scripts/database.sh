#!/usr/bin/env bash
set -xeo pipefail

if ! command -v docker >/dev/null 2>&1; then
  echo >&2 "Error: docker is not installed or not in PATH"
  exit 1
fi

if ! command -v sqlx >/dev/null 2>&1; then
  echo >&2 "Error: sqlx is not installed or not in PATH"
  exit 1
fi

DB_CONTAINER="${DB_CONTAINER:=postgres}"
DB_PORT="${DB_PORT:=5432}"
SUPERUSER="${SUPERUSER:=postgres}"
SUPERUSER_PW="${SUPERUSER_PW:=postgres}"

APP_USER="${APP_USER:=app}"
APP_USER_PW="${APP_USER_PW:=secret}"
APP_DB="${APP_DB:=shortlinks}"

function wait_for_postgres {
  # Loop until postgres is ready
  until docker exec "${DB_CONTAINER}" pg_isready -U "${SUPERUSER}" -h localhost; do
    echo >&2 "Postgres is still unavailable, sleeping for 1 second..."
    sleep 1
  done
  echo >&2 "Postgres is up and running on port: ${DB_PORT}"
}

if [ -z "${SKIP_DOCKER}" ]; then
  if ! docker ps -aq -f "name=${DB_CONTAINER}" | grep -q .; then
    docker run \
      --name "${DB_CONTAINER}" \
      --env "POSTGRES_USER=${SUPERUSER}" \
      --env "POSTGRES_PASSWORD=${SUPERUSER_PW}" \
      --publish "${DB_PORT}:5432" \
      --detach \
      postgres
    wait_for_postgres

    CREATE_QUERY="CREATE USER ${APP_USER} WITH PASSWORD '${APP_USER_PW}';"
    docker exec -it "${DB_CONTAINER}" psql -U "${SUPERUSER}" -c "${CREATE_QUERY}"

    GRANT_QUERY="ALTER USER ${APP_USER} CREATEDB;"
    docker exec -it "${DB_CONTAINER}" psql -U "${SUPERUSER}" -c "${GRANT_QUERY}"
  elif ! docker ps -q -f "name=${DB_CONTAINER}" | grep -q .; then
    docker start "${DB_CONTAINER}"
    wait_for_postgres
  fi
fi

export DATABASE_URL="postgres://${APP_USER}:${APP_USER_PW}@localhost:${DB_PORT}/${APP_DB}"
sqlx database create
sqlx migrate run
echo >&2 "Postgres has been migrated, ready to go!"
