# Rocket JSON API

Este é um projeto de estudo de uma **API REST** construída em [Rust](https://www.rust-lang.org/) usando o framework [Rocket](https://rocket.rs/). O objetivo principal é demonstrar o retorno de respostas no formato JSON, criar rotas de CRUD (Criar, Ler, Atualizar e Deletar) para uma entidade `cliente`, e implementar um sistema básico de autenticação via token JWT (JSON Web Token).

## 🚀 Como rodar o projeto

Certifique-se de ter o Rust e o Cargo instalados na sua máquina.

1. Clone o repositório ou navegue até a pasta do projeto:
   ```bash
   cd rocket-json
   ```
2. Inicie o servidor:
   ```bash
   cargo run
   ```
O servidor estará rodando localmente em `http://127.0.0.1:8000`.

## 📚 Endpoints Disponíveis

Você pode testar estas rotas usando ferramentas como **Insomnia**, **Postman** ou **cURL**. Lembre-se de adicionar o cabeçalho `Content-Type: application/json` quando enviar dados no corpo da requisição.

### 🏠 Rota Inicial
- **`GET /`**
  - **Descrição:** Retorna uma mensagem de boas-vindas simples.

### 🔐 Autenticação (Login)
- **`POST /logar`**
  - **Descrição:** Simula o login de um usuário e retorna um Token JWT. Este token pode ser utilizado para acessar rotas protegidas enviando-o no Header como `Authorization: Bearer <seu-token>`.
  - **Corpo (JSON):**
    ```json
    {
        "username": "admin",
        "password": "123"
    }
    ```

### 👥 CRUD de Clientes

⚠️ **Atenção:** Como se trata de um projeto de estudo mocardo, existem regras de validação engessadas (hardcoded) em algumas rotas.

- **`GET /clientes`**
  - Retorna uma lista de todos os clientes cadastrados (dados fictícios).

- **`GET /clientes/<id>`**
  - Retorna as informações do cliente específico correspondente ao ID informado.

- **`POST /clientes`**
  - Cria um novo cliente.
  - **Regra:** O campo `nome` deve ser exatamente `"Cliente Teste"`, caso contrário a API retornará um erro `400 Bad Request`.
  - **Corpo (JSON):**
    ```json
    {
        "nome": "Cliente Teste",
        "cpf": "123.456.789-00"
    }
    ```

- **`PUT /clientes/<id>`**
  - Atualiza um cliente existente.
  - **Regra:** Assim como na criação, exige que o campo `nome` seja `"Cliente Teste"`.
  - **Corpo (JSON):**
    ```json
    {
        "nome": "Cliente Teste",
        "cpf": "000.000.000-00"
    }
    ```

- **`DELETE /clientes/<id>`**
  - Deleta um cliente existente.
  - **Regra:** A exclusão só será bem-sucedida se o ID passado for `1`. Para qualquer outro ID, retornará erro `404 Not Found`.

## 🛠 Tecnologias utilizadas
- [Rust](https://www.rust-lang.org/)
- [Rocket Framework](https://rocket.rs/)
- Serde (para serialização e manipulação de JSON)
