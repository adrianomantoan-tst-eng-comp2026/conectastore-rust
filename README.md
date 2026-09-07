# ConectaStore — Sistema de Recomendação de Produtos Baseado em Grafos

Projeto acadêmico desenvolvido em **Rust** para a disciplina de Estruturas de Dados do curso de Engenharia da Computação.

O ConectaStore simula um sistema de recomendação de produtos para um ambiente de comércio eletrônico de grande escala, utilizando **grafos** e estruturas de dados complementares.

## Objetivo

O objetivo do projeto é demonstrar como grafos podem ser utilizados para representar relações entre clientes e produtos e, a partir dessas conexões, gerar recomendações relacionadas ao comportamento e aos interesses dos usuários.

## Modelagem do Grafo

O sistema utiliza dois tipos principais de vértices:

- `Client`: representa um cliente.
- `Product`: representa um produto.

As arestas representam relações entre os vértices, como:

- interesse ou compra de um produto por um cliente;
- similaridade ou relação entre produtos.

Cada aresta também possui um peso (`weight`), que pode representar a intensidade da relação.

## Estruturas de Dados

O projeto utiliza:

- `HashMap` para armazenamento e consulta eficiente de produtos;
- lista de adjacência para representação do grafo;
- `Vec` para armazenamento das conexões;
- `VecDeque` como fila para execução da busca em largura;
- `HashSet` para controlar vértices visitados e impedir recomendações duplicadas.

## Algoritmo de Recomendação

A recomendação é realizada utilizando **BFS (Breadth-First Search / Busca em Largura)**.

O algoritmo percorre o grafo por níveis a partir de um cliente ou produto.

Quando a busca começa por um cliente, os produtos diretamente associados a ele representam seu histórico ou seus interesses. O sistema percorre as relações desses produtos para encontrar novos produtos que possam ser recomendados.

O `HashSet` garante que um mesmo vértice não seja processado repetidamente, evitando recomendações duplicadas.

## Estrutura do Projeto

```text
conectastore-rust/
├── src/
│   ├── graph.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── performance.rs
│   ├── product.rs
│   └── recommendation.rs
├── tests/
│   └── integration_test.rs
├── Cargo.toml
└── README.md