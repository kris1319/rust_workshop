FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

RUN apt-get update && apt-get install -y --no-install-recommends python3-pip pex

RUN git clone https://github.com/facebook/folly -b v2025.08.25.00 /folly

WORKDIR /folly

# System fast-float is not fresh enough - avoid installing it from dpkg and build manually
RUN sed -i 's/fast_float//' /folly/build/fbcode_builder/manifests/folly \
    && sed -i 's/cmd_argss.append(\["pip", "install", "pex"\])//' /folly/build/fbcode_builder/getdeps.py

RUN python3 ./build/fbcode_builder/getdeps.py install-system-deps --recursive

RUN git clone https://github.com/fastfloat/fast_float.git -b v8.0.2 /fast_float \
    && cmake -S /fast_float -B /fast_float/build \
    && cmake --build /fast_float/build \
    && cmake --install /fast_float/build \
    && rm -rf /fast_float/build

RUN python3 ./build/fbcode_builder/getdeps.py --allow-system-packages build --no-tests \
    && python3 ./build/fbcode_builder/getdeps.py show-build-dir \
    && rm -rf $(./build/fbcode_builder/getdeps.py show-scratch-dir)/build

RUN mkdir -p /devcontainer
RUN ln -s $(python3 ./build/fbcode_builder/getdeps.py show-inst-dir)/include /devcontainer/folly-include
RUN ln -s $(python3 ./build/fbcode_builder/getdeps.py show-inst-dir)/lib /devcontainer/folly-lib
# Dependency - fmt
RUN ln -s $(python3 ./build/fbcode_builder/getdeps.py show-scratch-dir)/installed/fmt*/include /devcontainer/fmt-include
RUN ln -s $(python3 ./build/fbcode_builder/getdeps.py show-scratch-dir)/installed/fmt*/lib /devcontainer/fmt-lib
