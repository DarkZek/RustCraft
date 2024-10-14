FROM rustlang/rust:nightly

ARG SERVER_URL
ARG API_URL
ARG PUBLIC_JWT_KEY

# nightly wasm
RUN apt update
RUN curl -sL https://deb.nodesource.com/setup_20.x | bash
RUN apt install cmake golang libclang1 clang build-essential nginx nodejs -y
RUN rustup target add wasm32-unknown-unknown

# wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# npm

# Copy working files
COPY --from=repo . /build/
WORKDIR /build/client/

# Build executable
RUN wasm-pack build --target bundler --release --bin rc_client

RUN rm -Rf /build/site/wasm/*
RUN cp -R /build/client/pkg/* /build/site/wasm

WORKDIR /build/site

RUN npm i
RUN npm run build

RUN rm /var/www/html/*
RUN cp -R /build/site/dist/* /var/www/html

RUN cp /build/site/nginx.conf /etc/nginx/

RUN rm -Rf /build

CMD ["nginx", "-g", "daemon off;"]