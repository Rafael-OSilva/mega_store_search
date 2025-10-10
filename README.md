# 🛒 Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## 📖 Descrição do Projeto
Este projeto implementa um **sistema de busca otimizado** para o catálogo de produtos da empresa fictícia **MegaStore**, utilizando a linguagem **Rust** e estruturas de dados eficientes, como **tabelas hash** (`HashMap`).

O objetivo é simular um mecanismo de busca rápido e escalável, capaz de lidar com grandes volumes de dados e retornar resultados relevantes de forma eficiente.

---

## 🚀 Funcionalidades
- Cadastro (indexação) de produtos no catálogo.  
- Busca por:
  - **Nome** (palavras-chave)
  - **Marca**
  - **Categoria**
- Estrutura de índice baseada em **HashMaps** para acesso rápido.  
- Testes automatizados com o framework nativo do Rust.  
- Código modular e escalável, seguindo boas práticas da linguagem.  

---

## 🧩 Tecnologias Utilizadas
- **Linguagem:** Rust (Edition 2021)  
- **Gerenciador de pacotes:** Cargo  
- **Bibliotecas (crates):**
  - `serde` — serialização e deserialização de dados  
  - `serde_json` — manipulação de dados JSON  
  - `anyhow` — tratamento de erros simplificado  

---

## 🛠️ Estrutura do Projeto

megastore-search/
├── src/           # Código fonte do sistema
├── tests/         # Testes unitários e de integração
├── Cargo.toml     # Configuração do projeto e dependências
└── README.md      # Documentação do projeto
