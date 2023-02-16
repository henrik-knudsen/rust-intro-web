FROM postgres:15.2-alpine

ENV POSTGRES_PASSWORD=P4SSw0rd111
ENV POSTGRES_DB app
COPY init.sql /docker-entrypoint-initdb.d/