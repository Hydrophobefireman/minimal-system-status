default:
    @just --list

# Auto-format the source tree
fmt:
    cargo fmt

# Run 'cargo run' on the project
run *ARGS:
    cargo run {{ARGS}}

# Run 'cargo watch' to run the project (auto-recompiles)
watch *ARGS:
    cargo watch -x "run -- {{ARGS}}"


docker:
    #!/bin/bash -eux
    nix build .#dockerImage
    docker load < ./result


clean:
    rm -rf result
