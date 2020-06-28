# imagecodes

[![Rust](https://github.com/phansch/imagecodes/workflows/Rust/badge.svg)](https://github.com/phansch/imagecodes/actions?query=workflow%3ARust)

## Releasing a new version

Use `cargo release` from the `master` branch. This will do all the plumbing and
upload the release artifact to the GitHub releases.

The deployment script is then downloading the latest release and installs it on
the server.

## Deployment

To deploy a new release on the server, run

    ./deploy
