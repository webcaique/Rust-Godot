/*
# ENUM

enum ipVersion {
    v4,
    v6,
}

let version4 = ipVersion::v4;
let version6 = ipVersion::v6;


fn main() {
enum EnderecoIp {
    V4(String),
    V6(String),
}

let local = EnderecoIp::V4(String::from("127.0.0.1"));

let loopback = EnderecoIp::V6(String::from("::1"));
}


# FACILITA GUARDAR DETERMINADOS DADOS

fn main() {
enum EnderecoIp {
    V4(u8, u8, u8, u8),
    V6(String),
}

let local = EnderecoIp::V4(127, 0, 0, 1);

let loopback = EnderecoIp::V6(String::from("::1"));
}



fn main() {
enum Mensagem {
        Sair,
        Mover { x: i32, y: i32 },
        Escrever(String),
        MudarCor(i32, i32, i32),
    }
}

impl => também pode ser usado no enum


fn main() {
enum Mensagem {
    Sair,
    Mover { x: i32, y: i32 },
    Escrever(String),
    MudarCor(i32, i32, i32),
}

impl Mensagem {
    fn invocar(&self) {
        // o corpo do método é definido aqui
    }
}

let m = Mensagem::Escrever(String::from("olá"));
m.invocar();
}


# OPTION
Um enum em que o valor pode ser algo ou pode ser nada
Seria o "nulo" do rust


enum Option<T> {
    Some(T), // algum valor
    None, // nenhum valor
}

# Ainda pode usar o Some e None sem chamar explicitamente Option::
# <T> significa que o some pode conter qualquer tipo

let algum_numero = Some(5);
let algum_texto = Some("um texto");

let numero_ausente: Option<i32> = None;

Em resumo, é porque Option<T> e T (podendo T ser de qualquer tipo) são tipos diferentes, por isso, o compilador não vai permitir usar um valor do tipo Option<T> como se ele definitivamente tivesse um valor válido. Por exemplo, o código seguinte não vai compilar, porque ele está tentando somar um i8 a um Option<i8>.

# MATCH

enum Moeda {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn valor_em_cents(moeda: Moeda) -> u32 {
    match moeda {
        Moeda::Penny => 1,
        Moeda::Nickel => 5,
        Moeda::Dime => 10,
        Moeda::Quarter => {
            println!("MUEDA!");
            25
        },
    }
}



//////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
enum Estado {
   Alabama,
   Alaska,
}

enum Moeda {
   Penny,
   Nickel,
   Dime,
   Quarter(Estado),
}

fn valor_em_cents(moeda: Moeda) -> u32 {
    match moeda {
        Moeda::Penny => 1,
        Moeda::Nickel => 5,
        Moeda::Dime => 10,
        Moeda::Quarter(estado) => {
            println!("Quarter do estado {:?}!", estado);
            25
        },
    }
}


## USANDO O OPTION<T>


fn mais_um(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let cinco = Some(5);
let seis = mais_um(cinco);
let nenhum = mais_um(None);


# PLACEHOLDER

fn main() {
let algum_valor_u8 = 0u8;
match algum_valor_u8 {
    1 => println!("um"),
    3 => println!("três"),
    5 => println!("cinco"),
    7 => println!("sete"),
    _ => (),
}

_ => indica que não queremos cobrir todos os casos

# IF LET

Simplifica 

fn main() {
let algum_valor_u8 = Some(0u8);
match algum_valor_u8 {
    Some(3) => println!("três"),
    _ => (),
}

Para

fn main() {
let algum_valor_u8 = Some(0u8);
if let Some(3) = algum_valor_u8 {
    println!("três");
}

SImplifica
let mut contagem = 0;
match moeda {
    Moeda::Quarter(estado) => println!("Quarter do estado {:?}!", estado),
    _ => contagem += 1,
}

Para
let mut contagem = 0;
match moeda {
    Moeda::Quarter(estado) => println!("Quarter do estado {:?}!", estado),
    _ => contagem += 1,
}

## CAPTURANDO VALORES

enum IpAddress {
    V4(String),
    V6(String),
}

fn route(ip: IpAddress) {
    
    match ip {
        IpAddress::V4(addr) => {
            println!("IPV4 = {addr}");
        },
        IpAddress::V6(addr) => {
            println!("IPV6 = {addr}");
        }
    }
}






*/

fn main() {
    println!("Hello, world!");
}
