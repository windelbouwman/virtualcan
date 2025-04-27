
This folder contains a docker file to build a container with all required build tools installed.

Build the container:

    $ podman build --tag virtualcan-build-env --file Dockerfile

Test the image:

    $ podman run --tty --interactive --rm virtualcan-build-env
