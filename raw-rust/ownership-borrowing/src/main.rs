// Ownership => regras para gerenciamento de memória
/*
Um valor tem um owner e só pode ter um owner o tempo todo; quando owner descartado, o valor morre

TIPO STRING
São uma sequência de bytes alocados no heap
    let mut s = String::from("Hello"); // Se você conhece Java já deve ter visto isso. Se não, calma, estamos chamando a função `from` que pertence à 
                                       // String. Basicamente criando nossa String a partir de um literal
    s.push_str(", World!"); // adicionamos uma string literal a nossa string (na prática o texto é copiado e nossa String aumenta de tamanho)

let s1 = String::from("Hellow");
let s2 = s1;

s1 é endereço para a string, então s2 vai receber o endereço para a string
O valor de s1 é movido para s2

let s2 = s1.clone() // realmente acontece a copia

Borrowing e referências

// Referências seguem o formato "&T"
fn get_tamanho(s: &String) -> usize {
    s.len() // propriedades e métodos podem ser usados diretamente
}

get_tamanho(&s) // vai passar a referência e quando retornado, o s não é desalocado

Num borrowing, só pode ter um elemento mutável OU inúmeros elementos imutáveis

// ERRO
fn dangle() -> &String {
    let s = String::from("Hello");

    &s
}

Quando a função termina, s sai do escolo e é desalocada, logo returno um endereço inválido




*/

fn main() {
    let mut s = String::from("Texto");
    s.push_str(", TEXTO!");
    println!("{s}");
    println!("{}",fooo(&s));
    println!("{}",foo(s)); // vai retornar um owner, com isso, a variável s deixa de ter o owner
    let mut string: &String = dangle();
}


fn foo(s: String) -> String {
    s
}


fn fooo(s: &String) -> usize {
    s.len()
}
// &mut designa uma referência mutável
fn add_hello(s: &mut String) {
    s.push_str(" Hello"); // Esse método modifica a string e por isso precisamos de uma referência mutável, tente retirar o `mut` para ver o que acontece
}

fn dangle() -> &String {
    let s = String::from("Hello");

    &s
}

