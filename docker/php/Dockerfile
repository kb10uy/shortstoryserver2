FROM php:7.3-alpine
LABEL maintainer="kb10uy"

RUN apk add --no-cache postgresql postgresql-dev icu icu-dev \
    && docker-php-ext-install pgsql pdo_pgsql iconv intl bcmath \
    && apk del postgresql-dev icu-dev

WORKDIR /var/www
EXPOSE 8000
