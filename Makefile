docker_init:
	-docker network create lifesim_net >/dev/null 2>&1
	
run: halt docker_init prune
	-docker-compose -f ./docker-compose.yml up -d

build_run: halt docker_init prune
	-docker-compose -f ./docker-compose.yml up -d --build

halt:
	-docker-compose -f ./docker-compose.yml down

prune:
	-docker image prune -f