fn main() {
    if (true && false){
        println!("AQUI 1");
    } else {
        println!("AQUI 2");
    }
    let num = if false {5} else {6};
    println!("20 + 10 = {}", sum(20,10));
    println!("num = {}", num);
}

/*
WHILE => while (condicao) {}
LOOP => loop{} => infinito

FOR
for el in a {} => passa por um vetor a
for el in 0..10 {} => acesa o intervalo de 0 até 9
for el in 0..=10 {} => acesa o intervalo de 0 até 10
*/

fn sum(a:i32, b:i32) -> i32{
    a+b
    // No rust, caso seja a última linha e retornar um valor qualquer, não precisa declarar o return e não terminar com ponto e vírgula
    // SIMILAR: return a+b;
}