/*
# ORGANIZAÇÃO DE PROJETOS COM MÓDULOS
## PACOTES E CRATES
Na superfície, uma crate é a menor unidade de código que pode ser compilada e executada independentemente, seja usando o cargo ou o rustc. Dito isso, uma crate pode ser um executável ou uma biblioteca que pode ser usada por outros programas. A crate contém módulos e submódulos dentro dela como veremos daqui a pouco.

Já um package é uma coleção de crates relacionadas que podem ser publicadas e instaladas juntas. Um pacote é definido por um arquivo Cargo.toml, que especifica as dependências e outras informações do projeto. O pacote pode conter vários arquivos executáveis, mas geralmente apenas uma biblioteca.

O <cargo new meu_projeto> cria um pacote e um crate executável.
Cria uma biblioteca, usa-se <cargo new --lib meu_projeto>, criando um lib.rs (e não um main.rs)

## MÓDULO
O compilador cria o módulo raiz do nosso projeto, que parte do arquivo src/lib.rs ou src/main.rs, dependendo do tipo de crate. Esse módulo raiz é chamado de crate. Todos os módulos que criarmos serão filhos do módulo raiz. Sendo assim, declaramos módulos usando a palavra-chave mod seguida do nome do módulo, e.g. mod meu_modulo

mod say_hello {
    fn say_hello() {
        println!("Hello, world!");
    }
}

fn main() {
    say_hello::say_hello();
}

// Dará erro, pois o módulo é privado por padrão, proibindo sua visibilidade externa.
// Para resolver isso, declára como pub as funções

mod say_hello {
    pub fn say_hello() {
        println!("Hello, world!");
    }
}

fn main() {
    say_hello::say_hello();
}

## NAVEGANDO PELOS MÓDULOS
É possível o ivocar pelo crate (caminho absoluto), super ou self (caminhos relativos);
Navegamos mais fundo em uma hierarquia de módulos usando ::
// Criamos um módulo utils que tem um módulo math dentro dele

mod utils {
    pub fn say_hello() {
        say("Hello, world!");
    }

    // pub mod deixa apenas o nome do módulo visivel e não o módulo todo!
    pub mod math {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        // Exemplo com super
        pub fn hello_from_math() {
            super::say_hello();
        }
    }

    /* Exemplo com o self */
    fn add_two(a: i32) -> i32 {
        // refere-se ao próprio módulo
        self::math::add(a, 2)
    }

}

fn main() {
    crate::utils::say_hello();
    let x = utils::math::add(1, 2);
    println!("x = {}", x);
}

Os caminhos podem ficar um pouco longos de vez em quando, imagine só ter que escrever isso crate::modulo::submodulo::subsubmodulo::funcao. Para resolver isso temos a palavra-chave use que introduz no escopo do nosso programa o elemento que desejamos. Então se fizermos use crate::modulo::submodulo::submodulo::funcao;, vamos poder usar funcao livremente pelo nosso programa.

No topo da biblioteca src/lib.rs
// Precisamos deixar nossa inclusão pública para ser usada por outros além do escopo da biblioteca
pub use modulo::submodulo::funcao;

No lado do usuário 
use mylib::funcao;

fn main() {
    funcao();
}

Também temos outros dois operadores quando lidando com módulos. O * (operador glob) inclui tudo que o módulo contém em nosso escopo. Já o as converte o nome de uma inclusão em outra:
use std::io::Result as IOResult; // Renomeamos io::Result para IOResult
use std::fs::*; // Importa todos os elementos de fs

// Não faça isso
use std::io;
use std::io::Writer;
use std::io::Result;
// Faça isso
use std::io::{self, Writer, Result};

fn main() -> Result<()> {
    Ok(())
}

## STRUCT E ENUMS
Já com structs, tornar a declaração pública não torna nem seus métodos nem seus atributos públicos. Isso se deve, novamente, ao conceito de encapsulamento e o controle mais granular que você pode querer ter com suas structs.

mod Restaurante {
    // traits e enums 100% ficam publicos
    pub trait Comestivel {
        fn comer(&self);
    }

    pub enum Entradas {
        Salada,
        Sopa,
        Pao
    }

    pub struct Pizza {
        pub sabor: u32, // sabor pode ser acessado e editado
        tamanho: u32, // tamanho e preço não podem ser acessado do lado de fora do módulo Restaurante
        preco: f32
    }

    impl Pizza {
        // new pode ser acessado
        pub fn new() -> Self {
            todo!()
        }
        // aquecer não pode ser acessado de fora
        fn aquecer() {
            todo!()
        }
    }
}

### SEPARAR EM ARQUIVOS E PASTAS

mod restaurante {

    use std::fmt::Display;

    pub trait Comestivel {
        fn comer(self);
    }


    pub enum Sabor {
        Mussarela,
        Calabresa,
        Frango
    }

    impl Display for Sabor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Mussarela => write!(f, "Mussarela"),
                Self::Calabresa => write!(f, "Calabresa"),
                Self::Frango => write!(f, "Frango")
            }
        }
    }

    pub struct Pizza {
        sabor: Sabor,
        preco: f64
    }

    impl Pizza {
        pub fn new(sabor: Sabor, preco: f64) -> Self {
            Self {
                sabor,
                preco
            }
        }

        pub fn sabor(&self) -> &Sabor {
            &self.sabor
        }

        pub fn preco(&self) -> f64 {
            self.preco
        }
    }

    impl Comestivel for Pizza {
        fn comer(self) {
            println!("Comendo a pizza de {}. yum yum!", self.sabor)
        }
    }
}

use restaurante::{Sabor, Comestivel};

fn main() {
    let p = restaurante::Pizza::new(Sabor::Calabresa, 40.0);
    p.comer();
}

Movendo o módulo restaurante para restaurante.rs e na raiz da crate 
mod restaurante;

use restaurante::{Sabor, Comestivel};

fn main() {
    let p = restaurante::Pizza::new(Sabor::Calabresa, 40.0);
    p.comer();
}

SEPARANDO EM PASTAS E EM OUTROS ARQUIVOS:
src/
|   restaurante/
|   |   mod.rs
|   |   pizza.rs
|   |   sabor.rs
|   |   comestivel.rs
|   main.rs
target/
.gitignore
Cargo.lock
Cargo.toml

### EM CADA ARQUIVOS
-> mod.rs
mod comestivel;
mod sabores;
mod pizza;

// tornamos todas os elementos publicos para o externo
pub use comestivel::Comestivel;
pub use sabores::Sabor;
pub use pizza::Pizza;

-> sabores.rs
use std::fmt::Display;

pub enum Sabor {
    Mussarela,
    Calabresa,
    Frango
}

// implementamos o Display trait para que Sabor seja imprimivel na tela
impl Display for Sabor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mussarela => write!(f, "Mussarela"),
            Self::Calabresa => write!(f, "Calabresa"),
            Self::Frango => write!(f, "Frango")
        }
    }
}

-> comestiveis.rs
pub trait Comestivel {
    fn comer(self);
}

-> pizza.rs
use super::Sabor;
use super::Comestivel;

pub struct Pizza {
    sabor: Sabor,
    preco: f64
}

impl Pizza {
    pub fn new(sabor: Sabor, preco: f64) -> Self {
        Self {
            sabor,
            preco
        }
    }

    pub fn sabor(&self) -> &Sabor {
        &self.sabor
    }

    pub fn preco(&self) -> f64 {
        self.preco
    }
}

impl Comestivel for Pizza {
    fn comer(self) {
        println!("Comendo a pizza de {}. yum yum!", self.sabor)
    }
}

-> main.rs
mod restaurante;

use restaurante::{Sabor, Comestivel};

fn main() {
    let p = restaurante::Pizza::new(Sabor::Calabresa, 40.0);
    p.comer();
}


# INSTALAÇÃO DE DEPENDÊNCIA
cargo new project
cd project
cargo add rand

*/

fn main() {
    println!("Hello, world!");
}
