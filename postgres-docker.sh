mkdir -p data/postgresql/

exec docker run -d --name=postgres13 -p 5432:5432 -v postgres-volume:`pwd`/data/postgresql/ -e POSTGRES_PASSWORD=CAXw6zWg8inQ8A postgres
