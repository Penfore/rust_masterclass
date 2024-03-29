mod primeiro;
mod variaveis;
mod operadores;

use crate::utils::terminal::{exibir_menu, esperar_enter, limpar_tela};

pub fn executar() {
    loop {
        let itens = [
            "Primeiro Exemplo",
            "Variáveis - Imutáveis",
            "Variáveis - Mutáveis",
            "Variáveis - Constantes",
            "Variáveis - Shadowing",
            "Operadores - Aritméticos",
            "Operadores - Relacionais",
            "Operadores - Lógicos",
        ];

        let selecionado = exibir_menu("Fundamentos", &itens, false);

        limpar_tela();

        match selecionado {
            1 => primeiro::exemplo(),
            2 => variaveis::imutaveis(),
            3 => variaveis::mutaveis(),
            4 => variaveis::contantes(),
            5 => variaveis::shadowing(),
            6 => operadores::aritmeticos(),
            7 => operadores::relacionais(),
            8 => operadores::logicos(),
            _ => break,
        }

        esperar_enter();
    }
}
