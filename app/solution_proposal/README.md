# How to run postgres container

Build image using:

docker build -t post_db -f Postgres.Dockerfile .

Run container using:

docker run -p 5432:5432 --rm post_db