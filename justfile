build:
    cargo b -r

build-opt-unstable:
    cargo +nightly b -r -Z build-std
