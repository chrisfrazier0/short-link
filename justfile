# List available commands by default
_default:
  @just --list

# Start the database
db:
  ./scripts/database.sh

# Audit the dependencies for known vulnerabilities
audit:
   cargo audit \
    --ignore RUSTSEC-2023-0071

# Run general CI checks locally
check: audit
  @cargo fmt --check
  @cargo clippy -- -D warnings
  @cargo test
  @cargo sqlx prepare --workspace --check -- --all-targets

# Prepare the sqlx query cache
prepare:
  cargo sqlx prepare --workspace -- --all-targets

# Shutdown docker containers
stop:
  docker stop postgres
