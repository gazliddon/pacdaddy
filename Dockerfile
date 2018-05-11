FROM ubuntu

LABEL My node thing!

WORKDIR /root

EXPOSE 6500/tcp
EXPOSE 6502/tcp

RUN apt-get update
RUN apt-get install -y rustc
RUN apt-get install -y cargo
RUN apt-get install -y nodejs
RUN apt-get install -y npm
RUN apt-get install -y git
RUN apt-get install -y tmux
RUN npm install -g yarn
RUN mkdir pacdaddy

COPY EXPORT pacdaddy/
WORKDIR /root/pacdaddy
RUN cd server && cargo build
RUN yarn install && yarn run deploy


