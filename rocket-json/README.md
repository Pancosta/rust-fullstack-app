# E-commerce: Carrinho de Compras em Rust

Este é um projeto Fullstack construído em **Rust** usando o framework **Rocket**. O sistema é um e-commerce simples contendo as funcionalidades de listagem de produtos, adição e remoção de itens em um carrinho de compras, e um cadastro de clientes.

Diferente de uma simples API JSON, este projeto utiliza **Renderização do Lado do Servidor (SSR)**, entregando páginas web (HTML, CSS, JS) através de templates interativos.

## 🚀 Como executar o projeto

1. Certifique-se de que o Rust e Cargo estão instalados.
2. O projeto utiliza um banco de dados **SQLite**, que já vem configurado na pasta `db/rusqlite.db`.
3. Inicie o servidor:
   ```bash
   cargo run
   ```
4. Acesse no seu navegador: `http://127.0.0.1:8000/`

## 🛠 Tecnologias e Bibliotecas Utilizadas
- **[Rocket](https://rocket.rs/)**: Framework Web.
- **[Tera](https://keats.github.io/tera/)** (via `rocket_dyn_templates`): Motor de templates para renderização do HTML.
- **[Rusqlite](https://github.com/rusqlite/rusqlite)**: Interface para comunicação com o banco de dados SQLite.
- **[Chrono](https://crates.io/crates/chrono)**: Manipulação de datas e horários.
- **[Dotenv](https://crates.io/crates/dotenv)**: Gerenciamento de variáveis de ambiente.

## 📂 Estrutura do Projeto
- `src/controllers/`: Responsável por receber as requisições HTTP e orquestrar as respostas (renderizando templates).
- `src/repositorios/`: Camada de abstração do banco de dados (SQLite), onde as queries SQL são executadas.
- `src/models/`: Representação das entidades do banco (ex: `Cliente`, `Produto`, `Pedido`).
- `src/modelviews/` & `src/dtos/`: Estruturas para transferir dados formatados para a view ou validar entradas.
- `templates/`: Páginas e componentes em HTML, usando a sintaxe Tera (ex: `index.html.tera`).
- `static/`: Recursos visuais, arquivos CSS, fontes e scripts JS (ex: Bootstrap).
- `db/`: Arquivo físico do banco de dados SQLite e scripts de criação.
