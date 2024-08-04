RED='\033[0;31m'
NC='\033[0m'

ENV_PATH="./.env"
if [[ ! -f $ENV_PATH ]]; then
  printf "${RED}Use this command from the root of the project${NC}\n"
  exit -1
fi

source $ENV_PATH
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@0.0.0.0:5432/${POSTGRES_DB}"

while IFS= read -r line; do
  [[ -z "$line" || "$line" == \#* ]] && continue
  unset $(echo "$line" | cut -d '=' -f 1)
done < $ENV_PATH
