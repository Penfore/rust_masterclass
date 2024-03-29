pub fn imutaveis() {
    let x = 5;
    // x = 10; // Erro: Não é possível alterar o valor de uma variável imutável
    println!("x => {}", x);
}

pub fn mutaveis() {
    let mut x = 10;
    let y = x; // primitivos são copiados
    println!("x, y => {}, {}", x, y);

    x = 15;
    println!("x, y => {}, {}", x, y);
}

pub fn contantes() {
    const Z: i32 = 20;
    println!("O valor de Z é {}", Z);
}

pub fn shadowing() {
    let a = 25;
    println!("O valor de a é {}", a);

    // a = 30; // Erro: Não é possível alterar o valor de uma variável imutável

    let a = "Texto"; // Mas é possível fazer shadowing
    println!("O valor de a é {}", a);

    let a = [1, 2, 3, 4, 5]; // E também é possível mudar o tipo
    println!("O valor de a é {:?}", a);
}
