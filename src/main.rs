mod utils;
mod fundamentos;
mod tipos;
mod controle;
mod funcoes;

use std::process::exit;
use utils::terminal::{exibir_menu, limpar_tela};

fn main() {
    loop {
        let itens: [&str; 5] = [
            "Fundamentos",
            "Tipos",
            "Controle",
            "Funções",
            "Ownership"
        ];

        let selecionado = exibir_menu("Principal", &itens, true);

        limpar_tela();

        match selecionado {
            1 => fundamentos::executar(),
            2 => tipos::executar(),
            3 => controle::executar(),
            4 => funcoes::executar(),
            5 => println!("5"),
            _ => exit(0),
        }
    }

}
