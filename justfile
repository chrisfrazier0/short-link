# List available commands by default
_default:
  @just --list

# Start the database
db:
  ./scripts/database.sh

# Shutdown docker containers
stop:
  docker stop postgres

# Start the server with bunyan pretty-print
run: db
  @cargo run | bunyan

alias start := run

# Audit the dependencies for known vulnerabilities
audit:
   cargo audit \
    --ignore RUSTSEC-2023-0071

# Run general CI checks locally
check: audit
  @cargo fmt --check
  @SQLX_OFFLINE=true cargo clippy -- -D warnings
  @cargo test
  @cargo sqlx prepare --workspace --check -- --all-targets
  @SQLX_OFFLINE=true cargo +nightly udeps

# Prepare the sqlx query cache
prepare:
  cargo sqlx prepare --workspace -- --all-targets

cov:
  @SQLX_OFFLINE=true cargo llvm-cov --all-features --workspace --html \
    --ignore-filename-regex '(main\.rs)'
