FROM rust:latest as builder
LABEL Maintainer="Paul Delafosse <paul.delafosse@protonmail.com>"

WORKDIR /usr/src/bombadil

COPY ./src src
COPY ./Cargo.toml Cargo.toml

RUN cargo build --release

FROM ubuntu:latest
LABEL Maintainer="Paul Delafosse <paul.delafosse@protonmail.com>"

RUN apt -y update
RUN apt -y upgrade
RUN apt -y install bats

ARG user=tom
ARG home=/home/$user
ARG bombadil=$home/bombadil
ARG src=/usr/src/bombadil/target/release
ARG target=/usr/bin

COPY --from=builder $src/bombadil $target/bombadil
RUN useradd -ms /bin/bash tom

COPY bats-tests/tom_home/ $home/
RUN chown -R $user:$user $home
VOLUME $home

USER $user:$user
COPY --chown=$user bats-tests $bombadil/tests
WORKDIR $bombadil/tests

ENTRYPOINT ["bats", "tests.sh"]
