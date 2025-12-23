// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
// cargo build para compilar

mod player;

// importa os principais símbolos da API do godot
use godot::prelude::*;

// Cria uma estrutura vazia para nossa extensão
// Você pode nomeá-la como quiser
struct MyExtension;

// Marca nossa estrutura com o atributo #[gdextension]
// E implementa o trait ExtensionLibrary para ela
#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}