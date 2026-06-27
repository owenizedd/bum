# Containerized E2E test for bum.
# Builds the project in a clean environment and runs e2e-test.sh.
FROM node:22-bookworm

# Install Rust (required by napi-rs build step).
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Bun.
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"

WORKDIR /app
COPY . .

# Install Node dependencies (napi-rs CLI, optional native packages).
RUN bun install

CMD ["bash", "e2e-test.sh"]
