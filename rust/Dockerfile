FROM rust:1.61.0-slim AS build
ARG APP_PATH=/usr/src/app

WORKDIR ${APP_PATH}
ADD . ./

RUN cargo build --release

#--------------

FROM debian:buster-slim

ARG APP_PATH=/usr/src/app
ARG APP=gql-rust

ENV APP_USER=appuser

RUN rm -f /etc/localtime; ln -s /usr/share/zoneinfo/Asia/Jerusalem /etc/localtime

RUN groupadd $APP_USER && useradd -g $APP_USER $APP_USER && mkdir -p ${APP_PATH}

COPY --from=build ${APP_PATH}/target/release/${APP} ${APP_PATH}/${APP}
COPY --from=build ${APP_PATH}/config.toml ${APP_PATH}/

RUN chown -R $APP_USER:$APP_USER ${APP_PATH}

USER $APP_USER
WORKDIR ${APP_PATH}

EXPOSE 8080

CMD ["./gql-rust"]