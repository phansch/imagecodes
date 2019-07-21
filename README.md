# imagecodes

[![Build Status](https://travis-ci.com/phansch/imagecodes.svg?token=Px6VALkAQfGLfygptq2q&branch=master)](https://travis-ci.com/phansch/imagecodes)

## Releasing a new version

Start: 7:45

Use `cargo release`. This will do all the plumbing and upload the release
artifact to the GitHub releases.

The deployment script is then downloading the latest release and installs it on
the server.

## Deployment

To deploy a new release on the server, run

    ./deploy
