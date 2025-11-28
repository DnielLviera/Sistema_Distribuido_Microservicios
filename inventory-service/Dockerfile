FROM rust:1.70 as builder

WORKDIR /app

# Copiar archivos de dependencias
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build de la aplicación
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Instalar dependencias runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copiar binario desde builder
COPY --from=builder /app/target/release/inventory-service /app/inventory-service
COPY .env ./

# Crear archivos necesarios
RUN touch inventory.json

# Exponer puerto
EXPOSE 8002

# Comando de ejecución
CMD ["./inventory-service"]