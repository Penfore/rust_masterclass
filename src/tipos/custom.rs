use std::fmt;

pub fn exemplo_struct() {
    struct Usuario {
        nome: String,
        email: String,
        ativo: bool,
        idade: u8,
    }

    let usuario = Usuario {
        nome: String::from("João"),
        email: String::from("jjj@abcdmail.com"),
        ativo: true,
        idade: 25,
    };

    println!("usuario => {} {}", usuario.nome, usuario.email);
    println!("usuario => {} {}", usuario.ativo, usuario.idade);
}

pub fn exemplo_enum() {
    enum DiasDaSemana {
        Segunda,
        Terca,
        Quarta,
        Quinta,
        Sexta,
        Sabado,
        Domingo,
    }

    impl fmt::Display for DiasDaSemana {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                DiasDaSemana::Segunda => write!(f, "Segunda-feira"),
                DiasDaSemana::Terca => write!(f, "Terça-feira"),
                DiasDaSemana::Quarta => write!(f, "Quarta-feira"),
                DiasDaSemana::Quinta => write!(f, "Quinta-feira"),
                DiasDaSemana::Sexta => write!(f, "Sexta-feira"),
                DiasDaSemana::Sabado => write!(f, "Sábado"),
                DiasDaSemana::Domingo => write!(f, "Domingo"),
            }
        }
    }

    println!("dia => {}", DiasDaSemana::Domingo);
    println!("dia => {}", DiasDaSemana::Segunda);
    println!("dia => {}", DiasDaSemana::Terca);
    println!("dia => {}", DiasDaSemana::Quarta);
    println!("dia => {}", DiasDaSemana::Quinta);
    println!("dia => {}", DiasDaSemana::Sexta);
    println!("dia => {}", DiasDaSemana::Sabado);
}
