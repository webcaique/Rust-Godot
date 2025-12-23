// SINAIS
// INICIO
#[derive(GodotClass)]
#[class(init, base=Nodo2D)]
struct Player {
    hp: u32,
    base: Base<Nodo2D>
}

#[godot_api]
impl Player {
    #[signal]
    fn tomou_dano(dano:u32);
}

// Conectando sinais
// Para acessar os todos sinais registrados, usa-se o signals()
// Para inscrever em um sinal, existem 3 formas:

// --- Mesma instância/objeto
/*
Ou seja, estamos na mesma instância ou estrutura. Nesse caso utilizamos a função connect_self, passando a função que será executada quando 
nosso sinal for emitido. Vejamos o exemplo do jogador: sempre que tomarmos dano, temos que atualizar nossos pontos de vida.
*/

impl Player {
    fn on_tomou_dano(&mut self, dano: u32) {
        self.hp -= dano;
    }
} 

#[godot_api]
impl Player {
    #[signal]
    fn tomou_dano(dano: u32);
}

#[godot_api]
impl INode2D for Player {
    fn ready(&mut self) {
        self.signals()
            .tomou_dano()
            .connect_self(Self::on_tomou_dano);
    }
}

/*
Declaramos nossa função on_tomou_dano (é comum utilizar o prefixo on nesses casos), que atualiza a vida do jogador. Já na função ready, 
conectamos o nosso sinal com essa função. Assim dizemos que on_tomou_dano está inscrita para receber sinais de tomou_dano. Simples assim 
criamos nosso primeiro sinal.
*/

// ---Objetos diferentes
/*
Caso nossa função operadora esteja em outro objeto ou instância que seja diferente de self, precisamos usar a função connect_other. Isso 
pode ser útil quando uma ação que nosso entidade sofreu afeta entidades terceiras. Por exemplo, se temos um escudo equipado, queremos que o 
escudo tome dano, e não nós! Veja uma atualização do exemplo:
*/
fn ready(&mut self) {
    self.signals()
        .tomou_dano()
        .connect_other(&self.shield, Shield::on_tomou_dano);//Shield é outro objeto
}

// --- Funções estáticas/sem objetos
/*
Em último caso, digamos que nossa função não esteja atrelada a nenhum objeto ou seja uma função estática da nossa estrutura
(sem o atributo self). Nessa situação usamos o método connect.

*/
fn ready(&mut self) {
    self.signals()
        .tomou_dano()
        .connect(|amount: u32| {
            // escreve amount na saida padrao
            println!("Amout: {amout}");
        });
}

// Emitir sinais => método emit()

// impl Player {
// ...

fn foi_acertado(&mut self) {
    let amount = 10;
    self.signals().tomou_dano().emit(amount);
}