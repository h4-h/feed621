RED='\033[0;31m'
NC='\033[0m'

ENV_PATH="./.env"
[ -f $ENV_PATH ] || ENV_PATH="../.env"
[ -f $ENV_PATH ] || ENV_PATH="../../.env"
[ -f $ENV_PATH ] || (printf "${RED}Can not find .env file.${NC}\n" && exit -1)

source $ENV_PATH
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@0.0.0.0:5432/${POSTGRES_DB}"

while IFS= read -r line; do
  [[ -z "$line" || "$line" == \#* ]] && continue
  unset $(echo "$line" | cut -d '=' -f 1)
done < $ENV_PATH
