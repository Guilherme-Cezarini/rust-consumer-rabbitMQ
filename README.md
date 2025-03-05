# Rust RabbitMQ Consumer

Este projeto é um consumer de mensagens do RabbitMQ escrito em Rust. As mensagens recebidas são armazenadas em um banco de dados MongoDB.

## Requisitos

- Rust (recomendado: versão mais recente estável)
- Cargo (gerenciador de pacotes do Rust)
- Docker e Docker Compose
- RabbitMQ 

## Configuração

Antes de executar a aplicação, é necessário configurar as variáveis de ambiente. Um exemplo de configuração pode ser encontrado no arquivo `.env.example`.

Crie um arquivo `.env` baseado no `.env.example` e edite conforme necessário:

```sh
cp .env.example .env
```

## Como executar

1. Suba o MongoDB utilizando o Docker Compose:

   ```sh
   docker-compose up -d
   ```

2. Compile e execute a aplicação Rust:

   ```sh
   cargo run --release
   ```

## Considerações

- O MongoDB e o RabbitMQ é iniciado via Docker Compose.
- Utilize o `.env` para definir as credenciais e conexões necessárias.

