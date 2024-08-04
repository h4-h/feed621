RED='\033[0;31m'
NC='\033[0m'

if [[ ! -d "./docker" ]]; then
  printf "${RED}Use this command from the root of the project${NC}\n"
  exit -1
fi

if [[ "$1" != "dev" && "$1" != "prod" ]]; then
  printf "${RED}Usage: start.sh <dev | prod>${NC}\n"
  exit -1
fi

BASE_COMPOSE="./docker/docker-compose.yaml"
DEV_COMPOSE="./docker/docker-compose.dev.yaml"
PROD_COMPOSE="./docker/docker-compose.prod.yaml"
ENV_FILE="./.env"

[ "$1" == "dev" ] && docker compose --env-file ${ENV_FILE} -f ${BASE_COMPOSE} -f ${DEV_COMPOSE} up -w
[ "$1" == "prod" ] && docker compose --env-file ${ENV_FILE} -f ${BASE_COMPOSE} -f ${PROD_COMPOSE} up -d
