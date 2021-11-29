FROM gitpod/workspace-full

# Install custom tools, runtime, etc.
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
