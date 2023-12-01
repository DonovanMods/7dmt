set dotenv-load := true

default:
    @just --list

check:
    trunk check --fix --all

upgrade:
    trunk upgrade
    cargo upgrade
