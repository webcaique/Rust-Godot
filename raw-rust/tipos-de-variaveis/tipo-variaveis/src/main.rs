fn main() {
    let x = 10;// imutável
    let mut y = 20;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("w = {}", w);
    y = 15;
    println!("x = {}; y = {}", x, y);

    let x: u32 = 18;
    /*
    TIPOS INTEIROS
    No godot, os tipos inteiros podem variar entre signed (Todos os inteiros) e os unsigned (valores >= 0);
    Também pode ser definidos pelo tamanho (i = signed e u = unsigned)
    8 bits -> i8 ou u8;
    16 bits -> i16 ou u16;
    32 bits -> i32 ou u32;
    64 bits -> i64 ou u64;
    128 bits -> i128 ou u 128;
    arch -> iarch ou uarch (Vai depender da arquitetura do computador, x32 à 32-bits ou x64 à 64-bits).

    Os signed tem intervalos de valores: -(2^(n-1)) a (2^(n-1))-1 
    Os unsigned tem intervalos de valores: 0 a (2^n)-1.

    TIPO FLOAT
    f32: float para 32bits (float)
    f64: para 64bits (double)

    TIPO BOOLEANO -> entre true e false, o tipo bool

    TIPO CARACTERES
    Aspas simples => char
        let c = 'z';

        let emoji: char = '🦀';
    
    TIPO ARRAY
    Como em C, o array é de tamanho fixo e homogêneo (têm o tipo próprio)

    TIPO TUPLA (Equivale à dicionários em pytho)
        let a: [u32; 5] = [1, 2, 3, 4, 5]; //Declaração explícita

        let tup: (i32, f64, bool) = (600, 3.2, false);

        println!("{}", tup.0); // 600

        let (x, y, z) = tup; 

        println!("{}", y); // 3.2

    TIPO UNIT() => tupla vazia

    CONSTANTES
    São tipos variáveis imutáveis que podem ser declaradas globalmente
    */
}
