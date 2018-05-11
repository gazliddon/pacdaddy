FROM ubuntu

LABEL My node thing!

WORKDIR /root

EXPOSE 6502/tcp

RUN apt-get update
RUN apt-get install -y rustc cargo
COPY server server

WORKDIR /root/server
RUN cargo build


