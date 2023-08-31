FROM tiangolo/uwsgi-nginx-flask:python3.8-alpine-2023-08-28
LABEL maintainer="Remi Alvergnat <toilal.dev@gmail.com>"

COPY / /guessit-rest
RUN cd /guessit-rest && pip3 install -e .

ENV UWSGI_INI /guessit-rest/guessitrest/uwsgi.ini
WORKDIR /guessit-rest/guessitrest
