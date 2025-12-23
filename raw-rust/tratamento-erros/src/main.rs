/*

# TRATAMENTO DE ERROS
Erros recuperáveis, que o usuário pode recuperar ou tentar de novo, por exemplo o arquivo não encontrado
Erros irrecuperávies, que o usuário não consegue tentar novamente, como acesso de memória

ERROS IRRECUPÁVEIS COM panic!
o panic! pode ser chamado quando algo chame ela, ou por alguma função. Quando chamado, ele encerra o programa

ERROS RECUPERÁVEIS COM Result
enum Result<T, E> {
    Ok(T),
    Err(E)
}

// Importa a estrutura `File`, que permite manipulação de arquivos
use std:fs:File;

fn main() {
    let file = File::open("hello.txt");
    // retorna um Result, em que T é um tipo File, e o E do tipo std::io::Error

    // AVISA QUE OCORREU O ERRO, MAS PARA A EXECUÇÃO DO CÓDIGO
    let file = match file {
        Ok(f) => f, // Estamos capturando e retornando f
        Err(erro) => panic!("Problema em abrir o arquivo: {erro:?}"),
    }

    OU

    let file = match file {
        Ok(f) => f,
        Err(erro) => match erro.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problema criando o arquivo: {fc:?}"),
            },
            outro_erro => {
                panic!("Problema abrindo o arquivo: {outro_erro:?}");
            }
        }
    }
}

# Atalhos para pânico com Error

// UNWARP
use std::fs::File;

fn main() {
    // unwrap é equivalente ao match e panic! do primeiro exemplo, caso tenhamos um erro o programa entra em pânico.
    let file = File::open("hello.txt").unwrap()
}

// EXPECT
use std::fs::File;

fn main() {
    // A diferença do expect é que ele recebe uma string como argumento, e será mostrada junto com a mensagem de erro.
    // Recomenda-se que a mensagem diga o que o programador esperava acontecer
    let file = File::open("hello.txt").expect("arquivo deveria ser aberto");
}

// PROPAGAR ERROS
Empurrar o erro para frente até ser tratado

use std::fs::File;
use std::io::{self, Read};

fn read_msg_from_file() -> Result<String, io::Error> {
    // Tentamos abrir o arquivo, caso falhe propagamos o erro
    let file = File::open("hello.txt");
    let file = match file {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut msg = String::new();

    // Lemos o arquivo e retornamos nossa mensagem ou um erro
    match file.read_to_string(&mut msg) {
        Ok(_) => Ok(msg),
        Err(e) => Err(e)
    }
}

fn main() {
    let msg = read_msg_from_file().expect("");
    println!("{}", msg);
}

## REDUZINDO O CÓDIGO COM O OPERADOR ?
let file =  match File::open("hello.txt") {
    Ok(f) => f,
    Err(error) => return Err(error)
}
É IGUAL >> let file = File::open("hello.txt")?

/// REFATORANDO
use std::fs::File;
use std::io::{self, Read};

fn read_msg_from_file() -> Result<String, io::Error> {
    let mut msg = String::new();
    File::open("hello.txt")?.read_to_string(&mut msg)?;
    Ok(msg)
}

fn main() {
    let msg = read_msg_from_file().expect("");
    println!("{}", msg);
}

// REFATORANDO AINDA MAIS
use std::fs; // BIBLIOTECA QUE RESUME A OPERAÇÃO

fn read_msg_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt");
}

fn main() {
    let msg = read_msg_from_file().expect("");
    println!("{}", msg);
}

## TIPO OPTION
enum Option<T> {
    Some(T),
    None
}

// EXEMPLO

fn get_from_array(arr: &[i32], index: usize) -> Option<&i32> {
    arr.get(index)
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7];
    // Assert é um macro que faz uma comparação e entra em pânico se ela falhar
    assert!(None == get_from_array(&arr, 12)); // Passamos um index inexistente
    assert!(Some(&1i32) == get_from_array(&arr, 0)); // Nosso retorno vem embrulhado em um Some
}


*/

fn main() {
    let v = vec![1,2,4];

    v[99];
}
