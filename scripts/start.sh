RED='\033[0;31m'
NC='\033[0m'

if [[ "$1" != "dev" && "$1" != "prod" ]]; then
  printf "${RED}Usage: start.sh <dev | prod>${NC}\n"
  exit -1
fi

PATH_PREFIX="."
[ -d "./scripts" ] || ".."

BASE_COMPOSE="${PATH_PREFIX}/docker-compose.yaml"
DEV_COMPOSE="${PATH_PREFIX}/docker-compose.dev.yaml"
PROD_COMPOSE="${PATH_PREFIX}/docker-compose.prod.yaml"

[ "$1" == "dev" ] && docker compose -f ${BASE_COMPOSE} -f ${DEV_COMPOSE} up -w
[ "$1" == "prod" ] && docker compose -f ${BASE_COMPOSE} -f ${PROD_COMPOSE} up -d
