FROM rust:1.35

WORKDIR /usr/src/pastebin
COPY . .

RUN rustup override set nightly
RUN cargo build
RUN mkdir upload

ENV ROCKET_ENV=prod

CMD ["cargo", "run"]