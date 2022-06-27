FROM gitpod/workspace-postgres
FROM gitpod/workspace-rust

RUN sudo apt update \
 && sudo apt upgrade -y \
 && sudo apt install postgresql postgresql-client -y \
 && sudo rm -rf /var/lib/apt/lists/* \