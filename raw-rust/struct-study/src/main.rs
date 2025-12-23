/*
# STRUCT
Definida semelhante a C


struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i8, i8, i8);
struct Point(i8, i8, i8);

let black = Color(0,0,0);

UNIT-LIKE
Struct sem nenhum elemento, usados para os trait

struct User {
    username: &String,
    email: &String,
    sign_in_count: u64,
    active: bool,
}


println!("{:?}", var); // indica que quer depurar a struct
// ou {:#?} para deixar mais legível

#[derive(Debug)] //permite dar println! na struct abaixo dela
struct Rectangle {
    length: u32,
    width: u32,
}

# MÉTODOS
Métodos são funções, mas são chamadas no contexto da struct e seus primeiros parâmetros são sempre o &self (a isntância da struct chamada)

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

FUNÇÕES ASSOCIADAS
Quando está implementada no struct, mas não é um método
Usada normalmente para criar novas instâncias da struct

fn main() {
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}
}

let test = Rectangle::square(3);



*/
struct Usuario {
    nome: String,
    email: String,
    ativo: bool,
}

fn build(email: String, nome: String) -> Usuario {
    Usuario {
        nome,
        email,
        ativo: true,
        
    }
}

struct UserTest {
    name: &str,
    email: &str,
    pass: u32,
}

fn build2(email: String, nome: String, user: Usuario) -> Usuario {
    Usuario {
        nome,
        email,
        ..user,
    }
}

fn main() {
    let usuario1 = Usuario {
        nome: String::from("Nome bem impactante"),
        email: String::from("jogador@gmail.com"),
        ativo: true,
    };

    let user1 = UserTest {
        name: "Caique",
        email: "caique@usp.br",
        pass: 0123456789,
    }

    println!("{} ", user1.email);
}
