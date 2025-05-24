# Usa una imagen oficial de Rust para construir la app
FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

# Segunda etapa: imagen más ligera basada en Debian Slim
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && apt-get clean

# Crear directorio de trabajo y copiar archivos estáticos
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/bulletin-board-app /usr/local/bin/app
COPY --from=builder /usr/src/app/static ./static

EXPOSE 3000

CMD ["/usr/local/bin/app"]
