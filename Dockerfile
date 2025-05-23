# Usa una imagen oficial de Rust para construir la app
FROM rustlang/rust:nightly as builder


# Establece el directorio de trabajo dentro del contenedor
WORKDIR /usr/src/app

# Copia los archivos de tu proyecto
COPY . .

# Compila la aplicación en modo release
RUN cargo build --release

# Segunda etapa: imagen más ligera basada en Debian Slim
FROM debian:bookworm-slim

# Instala las dependencias necesarias para correr binarios compilados en Rust
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && apt-get clean

# Copia el binario compilado desde la etapa anterior
COPY --from=builder /usr/src/app/target/release/bulletin-board-app /usr/local/bin/app

# Expone el puerto que usa tu aplicación (ajústalo si es diferente)
EXPOSE 3000

# Comando por defecto para ejecutar la app
CMD ["app"]
