FROM gitpod/workspace-full-vnc

# Install custom tools, runtimes, etc.
# For example "bastet", a command-line tetris clone:
# RUN brew install bastet
#
# More information: https://www.gitpod.io/docs/config-docker/
USER gitpod
RUN apt update && apt install \
libsoup2.4-dev \
libatk1.0-dev \
&& apt clean && rm -rf /var/lib/apt/lists
