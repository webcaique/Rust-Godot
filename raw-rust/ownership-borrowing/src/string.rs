/*
# STRING E SLICE
## The slice type
usize é que a String interpreta para acessar seus valores
Mesmo se encontrar o endereço desejado, tem que ter o cuidado de não liberar a string e perder ela.
*/

fn main() -> () {

    let s = String::from("Hellow world");

    let palavra: usize = first_word(&s);
    println!("{palavra}");

    let (i1, i2): (usize, usize) = second_word(&s);

    println!("{} {}", i1, i2);

    println!("{}", &s[i1..i2])

}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes(); // converte a string em um array de bytes

    for (i, &item) in bytes.iter().enumerate() {
        // .iter() vai retornar uma coleção e o enumerate vai retornar a tupla com o índice e a referência do valor
        if item == b' '{
            // b' ' => tranforma a string literal em byte literal
            return i; // retorna a posição do espaço
        }
    }

    s.len()
}

fn second_word(s: &String) -> (usize, usize) {
    let bytes = s.as_bytes();
    let mut i1: usize = 0;
    let mut i2: usize = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if i1 == 0 {
                i1 = i+1;
            } else if i2 == 0 {
                i2 = i;
            } else {
                return (i1, i2);
            }
        }
    }

    i2 = s.len();
    
    if i1 == 0 {
        println!("SEM SEGUNDA PALAVRA");
        return (0,0);
    }

    return (i1, i2);


    
}