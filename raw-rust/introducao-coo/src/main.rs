/*
# INTRODUÇÃO À COMPUTAÇÃO ORIENTADA À OBJETO

/////////////////////////////////////////////
// Encontra o maior inteiro
fn largest_int(list: &[i32]) ->&i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Encontra o maior char
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
//////////////////////////////////////////////////

// PARA TODOS OS TIPOS

// Para genéricos usamos em geral uma única letra maiúscula para representar esse tipo
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

## TIPO GENÉRICO EM
- Structs
struct Point<T> {
    x: T,
    y: T
}

- Enums
enum Option<U> {
    Some(U),
    None
}

- Métodos
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

## TRAIT
O comportamento de um tipo é definido pelos seus métodos. Tipos diferentes podem ter os mesmos comportamentos se tiverem os mesmos métodos
Um trait é um meio de definir um grupo de métodos, e assim, um comportamento desejado para um tipo.

Para a comparação do tipo T (genérico) funcionar, precisa de um trait que permita isso

// exigindo que o tipo T implemente o trait PartialOrd
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

## CRIANDO TRAITS

// CÓDIGO BASE, SEM O TRAIT, SEM TRATAMENTO DO TIPO
/// Recebe um arquivo e o comprime e envia pela rede
fn compress_send<T>(file: &T) {
  let zip: String =  file.compress();
  send(zip); // Envia uma string para a rede
}

fn send(data: String) {
    // faz algo.
}

// CÓDIGO COM TRAIT
trait Compressable {
    fn compress(&self) -> String;
}

fn compress_send(file: &impl Compressable) {
    let zip: String = file.compress();
    send(zip);
}
    
// IMPLEMENTANDO PARA UM TIPO ESPECÍFICO
struct PDF {
    contents: String,
}

impl PDF {
    fn new(path: &str) -> Self {
        (...)
    }
}
// Implementando um trait para um tipo
impl Compressable for PDF {
    fn compress(&self) -> Self {
        // Imagine um algoritmo de compressão
        self.contents
    }
}

fn main() {
    let mypdf = PDF::new("/some/path");
    compress_send(mypdf);
}


// IMPLEMENTAÇÃO PADRÃO (DIVERSOS TIPOS)


trait Compressable {
    fn compress(&self) -> String {
        String::from("Algoritmo de compressão super-rápido")
    }
}

struct PDF;
// Ao fazer isso usamos a implementação padrão
impl Compressable for PDF {}

struct MP4;
impl Compressable for MP4 {
    // Sobrescrevendo a implementação padrão
    fn compress(&self) -> String {
        String::from("Algoritmo rápido para vídeos")
    }
}

fn main() {
    let pdf = PDF{};
    let video = MP4{};
    println!("{}", pdf.compress());
    println!("{}", mp4.compress());
}

// TRAIT COMO ATRIBUTO DE UMA STRUCT
O compilador precisa saber o tamanho, em bytes, de todos os tipos. O trait não tem tamanho, é apenas um comportamento.
Vamos guardar o trait dentro de um ponteiro inteligente:
Box<dyn T>, Box indica um ponteiro que vai guardar o dyn T, onde T é o trait.
Rust fará sua mágica, que envolve um conceito chamado dynamic dispatch, e agora você pode ter um atributo que aceita qualquer tipo que implemente um certo atributo. Isso também pode ser usado em tipos de retorno.

// Retornamos um tipo Result, que recebe um unit, e um ponteiro Box para uma implementação do trait `Error`
fn main() -> Result<(), Box<dyn std::error::Error> {
    Ok(()) // Cena dos próximos capítulos
}

Usando o macro derive
Já tinhamos visto o uso de macros com traits antes ao usarmos o macro #[derive()]. Agora é hora de ver o que ele faz. O macro derive, funciona como um decorador (se você já usou Python) em que ele recebe um trait e implementa suas funções para um tipo determinado. Vamos ver novamente o trait Debug:

// O trait Debug serve para imprimir na tela tipos em modo de depuração.
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn main(){
    let p1 = Point{x: 12, y: 24};
    println!("{:?}", p1); // Você verá na tela o seguinte resultado: `Point {x: 12, y: 24}`
}

// Default
#[derive(Default, Debug)]
struct Point {
    x: i32, // O valor padrão dos inteiros é 0.
    y: i32,
}
fn main(){
    let p1 = Point::default();
    println!("{:?}", p1); // Você verá na tela o seguinte resultado: `Point {x: 0, y: 0}`
}

// CLONE
Clone dá aos nossos tipos o método .clone(), que faz uma cópia completa do objeto, sem movê-lo (a famosa deep copy).

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point{x: 1, y: 1};
    let p2 = p1.clone();

    println!("{:?}", p1);
    println!("{:?}", p2);
}

*/

fn main() {
    println!("Hello, world!");
}
