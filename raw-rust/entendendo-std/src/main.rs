/*
# ENTENDENDO O STD
use std::fs; // importamos fs para nosso escopo

fn main() -> Result<(), Box<dyn std::io::Error>>{ // std pode ser usada sem importar
    println!("{}", fs::read_to_string("hello.txt")?); // para usarmos fs, precisamos importá-la
    Ok(())
}

## VEC<T> => lista sequêncial dinâmica
### Criando vec<T>
let vetor: Vec<u32> = Vec::new()
vec.push();
vec.pop();
println!("{:?}", v);

let vec = vec![1,2,4];

Pode-se criar o vec usando [T, N], em que T é um número, e N a quantidade de elementos no vetor
let vec = vec![2; 10]

Agora para acessar
let mut hundred = vec.get(100); // tenta pegar o elemento na posição 100, se der erro retorna um None
hundred:i32 = vec[100]; // tenta acessar o elemento, se não conseguir, retorna um panic

v.insert(1, 3); // na posição 1 insere o 3
v.remove(0); // remove elemento na posição 0

ITERANDO
let v = vec![100, 32, 57];
for i in &v {
    println("{i}");
}

Para mutáveis
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}

## STRING
Equivale ao Vec<u8>

let s1 = String::from("Oi");

let s2 = "Oi".to_string(); // Outro método de criação

println!("{}", s1.chars().nth(0).unwrap()); // acesso o 0-ésimo caractere

let s3 = String::from("Olá, tudo bem?");
println!("{}", &s3[0..4]); // experimente trocar para outros valores

// Iterando

// Por caracteres
for c in "Mundo".chars() {
    println!("{c}");
}

// Por bytes
for b in "Mundo".bytes() {
    println!("{b}");
}

#TIPO HASHMAP: ESTRUTURAS CHAVE VALOR
Guardar valores com as chaves correspondentes
HashMap<K, V>, mapeias as chaves do tipo K para valores do tipo V, usando a função hashing (algoritmo que produz uma chave única na maioria das vezes)
Usados para recuperar valores com base em um tipo qualquer

use std::collections::HashMap; // precisamos incluí-lo no nosso projeto

#[derive(Debug)]
struct Creature {
    health: u8,
    attack: u8,
    defense: u8
}

fn main() {
    let mut creatures = HashMap::new();

    // inserimos com a chave e o valor
    creatures.insert(String::from("goblin"), Creature{health: 50, attack: 30, defense: 20});
    creatures.insert(String::from("skeleton"), Creature{health: 60, attack: 50, defense: 42});

    let goblin = String::from("goblin");
    // e acessamos usando a chave
    println!("{:?}", creatures.get(&goblin).unwrap());
}
// iteração
for (key, value) in &creatures {
    println!("{}: {:?}", key, value);
}

## ATUALIZAR O VALOR NO HASH
### SOBRESCREVER O EXISTENTE
use std::collections::HashMap;

// Um hashmap de times com suas pontuações
let times = HashMap::new();

times.insert(String::from("Time Azul"), 10);
times.insert(String::from("Time Azul"), 15);

println!("{times:?}");

### ADICIONANDO SOMENTE SE A CHAVE NÃO EXISTIR. Para isso temos o método entry(), que retorna uma estrutura Entry que nos permite fazer esse tipo de operações:
use std::collections::HashMap;

let mut times = HashMap::new();

times.insert(String::from("Time Azul"), 10);

// Time Vermelho não existe, logo é criado com valor 0
times.entry(String::from("Time Vermelho")).or_insert(0);
// Time Azul já existe, logo apenas recebemos uma referência para o valor
times.entry(String::from("Time Azul")).or_insert(145);

println!("{times:?}");

### ATUALIZANDO BASEADO NO VALOR ANTERIOR. 
use std::collections::HashMap;

let text = "mundo velho mundo novo";

// criamo
let mut map: HashMap<&str, u32> = HashMap::new();

// Separamos as palavras por espaços
for word in text.split_whitespace() {
    // Se a chave já existe pegamos, senão criamos
    let count = map.entry(word).or_insert(0);
    *count += 1; // Incrementamos o valor da chave
}

println!("{map:?}");

# ENTRADA E SAÍDA => std::io
read() => ler
write() => escrever

## Lendo um arquivo
use std::io;
use std::io::prelude::*; // Usando o wildcard * importamos tudo dentro do módulo para nosso escopo
use std::fs::File;

fn main() -> io::Result<()> { // Estrutura equivalente à Result<(), Box<dyn io::Error>>
    let f = File::open("hello.txt");
    let mut buffer = [0; 10];

    // n armazena quantos bytes foram lidos
    let n = f?.read(&mut buffer)?;
    println!("bytes: {:?}", &buffer[..n]);
    Ok(())
}

### ESCREVENDO A SAÍDA PADRÃO
use std::io;

fn main() -> io::Result<()> {
    // Inclusive aqui vai um uso do trait `Write`
    io::stdout().write(&[42])?;
    Ok(())
}

## LENDO A ENTRADA PADRÃO
use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new(); // Alocamos uma string para receber nosso input
    io::stdin().read_line(&mut input)?; // Lemos a stdin, escreva algo e aperte Enter

    println!("Input: {}", input.trim()); // removemos espaços no começo e no fim antes de printar
}

## COPY => copia os valores de um buffer para o outro
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    // Inclusive aqui vai um uso do trait `Write`
    io::stdout().write(&[42])?; // imprime de acordo com a tabela ASCII (42 == *)
    let mut input = String::new(); // Alocamos uma string para receber nosso input
    io::stdin().read_line(&mut input)?; // Lemos a stdin, escreva algo e aperte Enter

    println!("Input: {}", input.trim()); // removemos espaços no começo e no fim antes de printar
    
    io::copy(&mut io::stdin(), &mut io::stdout())?; // Vai copiar toda a entrado do stdin para a saída padrão

    io::stdin().read_line(&mut input)?; 
    Ok(())
}

## MANIPULANDO ARQUIVOS
### CRIAÇÃO
fn main() -> std::io::Result<()> {
    let mut file = File::create("hello.txt")?;
    file.write_all(b"Olá, mundo!")?;
    Ok(())
}

### CRIAÇÃO DE PASTAS DE MANEIRA RECURSIVA
use std::fs::{self, DirBuilder};

let path = "./tmp/foo/bar/baz";
DirBuilder::new()
    .recursive(true)
    .create(path).unwrap();

assert!(fs::metadata(path).unwrap().is_dir()); // Checamos se as pastas foram criadas

# PONTEIRO INTELIGENTE
## PONTEIRO Box<T>
Aloca memória no heap, ponteiro para o heap, usado quando:
Não se sabe o tamanho máximo;
Transferir a ownership de uma grandes quantidades de dados e garantir que os dados sejam movidos e não copiados;
Possuir (own) um valor que implementa um trait e só queremos saber sobre o trait implementado (o padrão Box<dyn T> onde T é um trait).

### USOS
fn main() {
    let b = Box::new(10); // 10 será armazenado no heap
    println!("{b}"); // veja 10 na tela
}

#### LISTA LIGADA
enum List {
    Nil,
    Node(i32, List)
}

Acaba que gera uma estrutura infinita, pois o List tem um Node, que tem um endereço  para uma List...
Além disso, pode estourar a memória no stack, precisando usar a memória do heap
Box, sendo um ponteiro:

enum List {
    Nil,
    Node(i32, Box<List>),
}

fn main() {
    let list = List::Node(1, Box::new(List::Node(2, Box::new(List::Nil))));
}

O Box é simples, mas é possível impletar os traits Deref e Drop, permitindo o Rust gerenciar a memória alocada

## PONTEIRO Rc<T>
O Box<T> não premite compartilhar os valores entre várias referências.
Em um caso que precisamos de possuir (own) múltiplas referências para o mesmo valor, como por exemplo em um grafo, onde vários nós podem apontar para um outro nó e vice-versa. É preciso outra ferramenta: o ponteiro inteligente Rc<T>.

Rc é um acrônimo para reference counting, que é uma forma de gerenciamento de memória que permite que várias referências compartilhem o mesmo valor armazenado em memória. Ele faz isso mantendo uma contagem de referências para o valor armazenado, incrementando a contagem quando uma variável é criada apontando para o mesmo valor e decrementando a contagem quando uma variável que apontava para ele é descartada. Quando a contagem de referências chega a zero, o valor é descartado da memória.

use std::rc::Rc;

enum List {
    Nil,
    Node(i32, Rc<List>),
}

use crate::List::{Node, Nil}; // Veremos isso mais a frente, mas estamos incluindo as variâncias de List no nosso escopo, para não ficar escrevendo List::Node ou List::Nil

fn main() {
    let a: Rc<List> = Rc::new(Node(1, Rc::new(Node(2, Rc::new(Nil)))));
    let b = Node(3, Rc::clone(&a));
    let c = Node(4, Rc::clone(&a));
}

Criamos uma lista a dentro de um Rc<List>, depois adicionamos um nó b que aponta para a, e um nó c que aponta para a. veja que usamos um método de Rc chamado clone, que cria uma nova referência para o valor armazenado em a e incrementa um contador interno. Ao final do programa, quando cada referência sai do escopo, o contador zera e nossa lista é liberada.

Isso resolve nosso problemas, porém Rc<T> também tem um porém, só podemos produzir referências imutáveis, seguindo as regras de borrowing. Para criar mutiplas referências mutáveis, vamos precisar da ajuda de outro ponteiro.

## PONTEIRO RefCell<T>
Usado para mutar dados através da referência.
Representa a posse singular, em que as regras de borrowing são checadas e cumpridas em tempo de execução e não de compilação.

Para referências imutáveis e mutáveis usamos & e &mut, respectivamente. Com RefCell<T> usamos os métodos borrow e borrow_mut, que devolvem respectivamente, o ponteiros inteligentes Ref<T> e RefMut<T>, ambos funcionam como referências normais mas como dito anteriormente as regras são impostas em tempo de execução. Sendo assim, violar essas regras resulta em um pânico e nosso programa se encerrará imediatamente.

use std::cell::RefCell;

fn main() {
    // Criamos um vetor dentro de um RefCell
    // Note que v NÃO é mutável
    let v: RefCell<Vec<i32>> = RefCell::new(vec![]);

    {
        // Criamos um novo escopo dentro do programa e pegamos v emprestado
        // Agora sim em sua versão mutável, aqui a mutabilidade interna
        let mut b = v.borrow_mut();
        b.push(5); // Adicionamos alguns valores
        b.push(10);
        b.push(15);
    } // b sai de escopo (as regras de borrowing ainda valem!)

    // Pegamos v emprestado e iteramos sobre
    let a = v.borrow();
    for i in a.iter() {
        print!("{} ", i);
    }
    println!()
}

*/

use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug)]
enum List{
    Node(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Node, Nil};

fn main() {
    let num = Rc::new(RefCell::new(1));

    let a = Rc::new(Node(num.clone(), Rc::new(Nil)));
    // Criamos uma lista com duas pontas apontando para a
    let b = Node(Rc::new(RefCell::new(2)), a.clone());
    let c = Node(Rc::new(RefCell::new(3)), a.clone());

    // Alteramos a
    *num.borrow_mut() += 10;

    println!("a: {a:?}");
    println!("b: {b:?}");
    println!("c: {c:?}");
}