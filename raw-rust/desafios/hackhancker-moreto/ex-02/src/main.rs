use std::io;

fn main() {
    let int1: i32;
    let int2: i32;
    let float1: f32;
    let float2: f32;

    let mut input = String::new();

    println!("Digite um número inteiro: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler a entrada!");

    int1 = input
        .trim()
        .parse()
        .expect("Digite um número válido!");
    
    input.clear();
    /*
    Input não é sobreescrito toda vez que é escrito
    pelo stdin.
    Por isso, usar o clear.
    */

    println!("Digite outro número inteiro: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Digite um número válido!");
    
    int2 = input
        .trim()
        .parse()
        .expect("Digite um número válido!");
    
    input.clear();

    println!("Digite um ponto flutuante (usando '.'):");
    io::stdin()
        .read_line(&mut input)
        .expect("Digite um float válido!");
    
    float1 = input
        .trim()
        .parse()
        .expect("Digite um número válido!");
    
    input.clear();

    println!("Digite outro ponto flutuante (usando '.'):");
    io::stdin()
        .read_line(&mut input)
        .expect("Digite um float válido!");

    float2 = input
        .trim()
        .parse()
        .expect("Digite um número válido!");

    
    println!("INTEIRO");
    println!("Soma: {}; Diferença: {}", (int1+int2), (int1-int2));
    println!("FLOAT");
    println!("Soma: {}; Diferença: {}", (float1+float2), (float1-float2));
    

}
