FROM gitpod/workspace-full-vnc

# Install custom tools, runtimes, etc.
# For example "bastet", a command-line tetris clone:
# RUN brew install bastet
#
# More information: https://www.gitpod.io/docs/config-docker/
USER gitpod
RUN sudo apt-get update -y && DEBIAN_FRONTEND=noninteractive sudo apt-get install -y --no-install-recommends \
libsoup2.4-dev \
libatk1.0-dev \
libpango1.0-dev \
libgtk-3-dev \
&& sudo apt-get clean -y && sudo rm -rf /var/lib/apt/lists

RUN sudo apt-get update -y && sudo apt-get install -y --no-install-recommends libwebkit2gtk-4.0-dev
