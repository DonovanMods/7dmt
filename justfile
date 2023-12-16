set dotenv-load := true

default:
    @just --list

check:
    trunk check --fix
    cd modinfo && trunk check --fix

upgrade:
    cargo upgrade
    trunk upgrade
    cd modinfo && cargo upgrade && trunk upgrade
