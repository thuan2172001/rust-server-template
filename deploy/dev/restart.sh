docker-compose --env-file .env -f docker-dev.yml up -d --build --force-recreate
./log.sh
