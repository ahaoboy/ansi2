default:
  just --list

bloaty-build:
  cargo build --profile bloaty --features="cli"
bloaty-csv:
  bloaty ./target/bloaty/ansi2 -d sections,symbols -n 0 --csv > meta.csv
bloaty-json:
  bloaty-metafile meta.csv --no-sections > meta.json
bloaty: bloaty-build bloaty-csv bloaty-json

clippy:
  cargo clippy --fix --allow-dirty --allow-staged --all-targets
fmt:
  cargo fmt
check: fmt clippy