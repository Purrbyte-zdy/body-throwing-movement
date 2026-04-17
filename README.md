# body-throwing-movement

A production-style Rust CLI for projectile motion analysis.

Given:
- launch height (`h0`)
- landing height (`h1`)
- horizontal distance (`d`)

the tool scans a configurable angle range and computes the required launch speed for each feasible angle using classical mechanics.

It also reports:
- the **slowest feasible launch speed** and its angle
- the **fastest feasible launch speed** and its angle

## Physics model

For each angle `theta`, required launch speed is:

```text
v0 = sqrt(g * d^2 / (2 * cos(theta)^2 * (d * tan(theta) - (h1 - h0))))
```

Only angles with a valid positive denominator are feasible.

## Features

- Rust CLI with clear argument validation
- Feasible angle-speed table output
- Slowest/fastest speed summary
- Optional CSV export
- Optional speed-vs-angle plot export (`.png` or `.svg`)
- Unit tests + integration test
- CI checks (fmt, clippy, tests, release build)
- Release workflow with multi-platform packaged binaries

## Quick start

```bash
cargo run -- \
  --launch-height 1.8 \
  --landing-height 1.0 \
  --distance 20 \
  --angle-min 5 \
  --angle-max 85 \
  --angle-step 0.5 \
  --csv-out output/results.csv \
  --plot-out output/speed-angle.png
```

## CLI options

```text
--launch-height   Launch height in meters
--landing-height  Landing height in meters
--distance        Horizontal distance in meters
--angle-min       Minimum scanned angle in degrees (default: 1)
--angle-max       Maximum scanned angle in degrees (default: 89)
--angle-step      Angle step in degrees (default: 0.5)
--gravity         Gravitational acceleration in m/s^2 (default: 9.80665)
--csv-out         Optional CSV output path
--plot-out        Optional chart output path (.png or .svg)
```

## Project structure

```text
src/
  cli.rs                  # CLI arguments
  error.rs                # Error types
  main.rs                 # Program entry
  physics/
    ballistics.rs         # Projectile solver
  output/
    table.rs              # Console + CSV output
    plot.rs               # PNG/SVG visualization
tests/
  cli.rs                  # Integration test
.github/workflows/
  ci.yml                  # CI pipeline
  release.yml             # Tagged release packaging
```

## Development

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo run -- --help
```

## CI/CD

- `CI` workflow runs on push/PR and performs:
  - formatting check
  - clippy linting
  - tests
  - release build sanity check
- `Release` workflow runs on tags like `v0.1.0` and:
  - builds Linux/macOS/Windows binaries
  - packages archives with README and LICENSE
  - generates SHA256 checksums
  - publishes files to GitHub Releases
