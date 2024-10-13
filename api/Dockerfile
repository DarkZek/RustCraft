FROM rustlang/rust:nightly

# Copy working files
WORKDIR /build
COPY . .

# Build executable
RUN cargo build --release

RUN mkdir /api

# Copy exe over
RUN cp /build/target/release/api /api/api

RUN rm -Rf /build

WORKDIR /api/

CMD ["./api"]