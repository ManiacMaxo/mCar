FROM debian

RUN apt update && apt install -y --no-install-recommends \
    git curl ca-certificates libopencv-dev libclang-dev clang && \
    update-alternatives --set c++ /usr/bin/clang++ && \
    update-alternatives --set cc /usr/bin/clang


RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
