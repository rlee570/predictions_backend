FROM postgres:latest

SHELL ["/bin/bash", "-c"]

RUN apt update && apt upgrade -y \
    && apt install curl -y \
    && apt install build-essential -y \
    && apt install libpq-dev -y  \
    && apt install libssl-dev -y \
    && apt install openssl -y \
    && apt install pkg-config -y
WORKDIR /usr/src/zuhlke_predictions_backend
COPY . .
CMD su - postgres -c '/usr/lib/postgresql/12/bin/initdb -D /var/lib/postgresql/data' \
    && su - postgres -c '/usr/lib/postgresql/12/bin/pg_ctl -D /var/lib/postgresql/data -l logfile start' \
    && su - postgres -c 'cd /usr/src/zuhlke_predictions_backend && psql -h 127.0.0.1 -f init.sql' \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && source "$HOME/.cargo/env" \
    && whereis rustc \
    && whereis rustup \
    && rustup default nightly \
    && cargo install diesel_cli --no-default-features --features postgres \
    && diesel setup \
    && diesel migration run \
    && cargo install --path . \
    && su - postgres -c 'cd /usr/src/zuhlke_predictions_backend && psql predictions -h 127.0.0.1 -f dummy.sql' \
    && zuehlke_predictions_backend