use std::collections::HashMap;

#[derive(Debug)]
struct Produto {
    nome: String,
    categoria: String,
    preco: f64,
}

fn main() {

    let mut produtos = HashMap::new();

    produtos.insert(
        1,
        Produto {
            nome: String::from("Notebook Gamer"),
            categoria: String::from("Eletrônicos"),
            preco: 4500.00,
        },
    );

    produtos.insert(
        2,
        Produto {
            nome: String::from("Mouse Gamer"),
            categoria: String::from("Acessórios"),
            preco: 150.00,
        },
    );

    produtos.insert(
        3,
        Produto {
            nome: String::from("Teclado Mecânico"),
            categoria: String::from("Acessórios"),
            preco: 350.00,
        },
    );

    println!("Produtos cadastrados:");

    for (id, produto) in &produtos {
        println!(
            "ID: {} | Nome: {} | Categoria: {} | Preço: R${}",
            id,
            produto.nome,
            produto.categoria,
            produto.preco
        );
    }

    println!("\nBuscando produto ID 2...\n");

    match produtos.get(&2) {
        Some(produto) => {
            println!("Produto encontrado:");
            println!("Nome: {}", produto.nome);
            println!("Categoria: {}", produto.categoria);
            println!("Preço: R${}", produto.preco);
        }

        None => {
            println!("Produto não encontrado.");
        }
    }
    println!("\nBuscando produto pelo nome...\n");

    for (_id, produto) in &produtos {

        if produto.nome.contains("Notebook") {

            println!("Produto encontrado:");
            println!("Nome: {}", produto.nome);
            println!("Categoria: {}", produto.categoria);
            println!("Preço: R${}", produto.preco);
        }
    }
     println!("\nSistema otimizado com HashMap em Rust.");
    println!("Busca rápida e eficiente para grandes catálogos.");
}