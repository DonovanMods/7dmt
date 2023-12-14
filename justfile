set dotenv-load := true

default:
    @just --list

check:
    trunk check --fix
    cd modlet && trunk check --fix --exclude clippy
    cd modinfo && trunk check --fix

upgrade:
    cargo upgrade
    trunk upgrade
    cd modlet && cargo upgrade && trunk upgrade
    cd modinfo && cargo upgrade && trunk upgrade
