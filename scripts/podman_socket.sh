RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

[ -z "$(command -v podman)" ] && (printf "${RED}No podman executable found${NC}\n" && exit -1)

printf "${GREEN}Starting podman socket${NC}\n"

podman system service -t 0 &
