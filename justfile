bt := '0'

export RUST_BACKTRACE := bt

watch:
    cargo watch --clear --exec test

check:
    cargo watch --clear --exec check
