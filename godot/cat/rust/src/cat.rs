use godot::classes::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)] // init vai criar um constructor com valores padrões
struct Cat {
    name: String, // ""
    #[init(val = 2)] //setando um valor para age = 2
    age: i32, // 0
    base: Base<Node2D> // criado com o construtor da classe base
}

#[godot_api]
impl INode2D for Cat {
    fn to_string(&self) -> GString {
        let Self {name, age, ..} = &self;
        format!("Cat(name={name}, age={age})").into()
    }
}

#[godot_api]
impl Cat {

    #[func]
    // pode ser chamado no GDScript
    fn get_name(&self) -> GString {
        self.name.clone()
    }

    // acessando métodos de classe base
    pub fn apply_movement(&mut self, delta: f32) {
        let pos = self.base().get_position();

        self.base_mut().set_postion(post+self.velocity * delta)
    }

    pub fn is_inside_area(&self, rect: Rect2) -> String {
        let node_name = self.base().get_name();

        format!("Cat(name={}, velocity={})", node_name, self.velocity)
    }

    fn does_something(&self, registry: &mut HashMap<String, Gd<Cat>>) {
        // pegando uma referênca Gd de self
        let self_as_gd: Gd<Self> = self.to_gd();

        // passando para outro método
        registry.insert(self.name.clone(), self_as_gd);
    }

}

impl INode2D for Cat {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            name: "EU".to_string(),
            age: 2,
            base,
        }
    }
}

/* Desabilitando o construtor
#[derive(GodotClass)]
#[class(no_init, base=Nodo2D)]
struct Cat {
    name: String,
    age: i32,
    base: Base<Node2D>
}
*/

// Construtores personalizados => retorna um Gd<Self>
//  - Se houver base
#[godot_api]
impl Cat {
    #[func]
    fn from_name_age(name: GString, age: i32) -> Gd<Self> {
        //chamando from _init_fn() e já retornando o resultado
        Gd::from_init_fn(|base| {
            // a base é do tipo Base<Node2D> e é redirecionada direto para o atributo base
            Self {
                name: name.into(),
                age,
                base
            }
        })
    }
}

//  - Sem base => Gd::from_object()
#[derive(GodotClass)]
#[class(no_init)]
// como não temos um atributos base, nossa classe base default será RefCounted
struct Cat1 {
    name: String,
    age: i32,
}

#[godot_api]
impl Cat1{
    #[func]
    fn create(name: GString, age: i32) -> Gd<Self> {
        Gd::from_object( Self {
            name: name.into(),
            age,
        })
    }

}

// Detrutores => Drop
/*
impl Drop for Monster {
	// Aqui você pode colocar alguma lógica para ser aplicada quando seu objeto for destruído/sair de escopo
    fn drop(&mut self) {
        godot_print!("O gatingo '{}' está indo para um lugar melhor da memória...", self.name);
    }
}
*/

// RESGISTRANDO PROPRIEDADES
/*
Conhecimentos até agora, precisa criar os getters e setters para cada propriedade
    #[var]: faz a propriedade ficar acessível no GDScript, tanto diretamente quanto através de getters e setters.
    #[export]: faz a propriedade ficar acessível pela UI do Godot.

    // usando #[var]
    #[derive(GodotClass)]
    #[class(init, base=Node2D)]
    struct Cat {
        #[var]
        name: GString, // precisa virar GString para Interagir diretamente com o GDScript
        age: i32,
        base: Base<Node2D>,
    }

    // No GDScript
        var cat = Cat.new()

        # mudando a propriedade diretamente
        cat.name = "Kira"
        # lendo a propriedade diretamente
        print(cat.name) # printa "Kira"

        # mudando a propriedade com o setter padrão
        cat.set_name("Gatito")
        # lendo a propriedade com o getter padrão
        print(cat.get_name()) # printa "Gatito"

    // usando #[export]
    #[derive(GodotClass)]
    #[class(init, base=Nodo2D)]
    struct Cat {
        #[export]
        name: GString,
        age: i32,
        base: Base<Node2D>
    }
*/

// ENUMS
/*
É possível também usar enums como propriedades de uma classe (na UI os valores possíveis do enum aparecerão em um menu drop-down).
Contudo, para fazer isso, precisamos de alguns passos extra, que estão explicados neste exemplo:

*/

// Usando o macro derive para definir alguns comportamentos
// GodotConvert: permite a conversão do tipo Food para algum tipo que o Godot entenda
// Var: nosso enum pode ser usado em atributos Var
// Export: nosso enum pode ser usado em atributos Export
// Default: diz que nosso enum tem um valor padrão, indicado por #[default]
#[derive(GodotConvert, Var, Export, Default)]
// Como o Godot não possui um tipo Enum, temos que dizer para ele como interpretar os valores
// Neste caso como string, mas poderia ser como número
#[godot(via = GString)]
pub enum Food {
	#[default] // Fish será o valor default
    Fish,
    Milk,
    Cheese,
}

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Cat {
    #[export]
    favorite_food: Food,
}

// CONSTATANTE (Godot só aceita do tipo inteiro)
#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Cat {
    #[constant]
    const NUM_OF_LIVES: i32 = 7,
    // ...
}

/*
var idk = Cat.NUM_OF_LIVES
*/

// REGISTRANDO FUNÇÕES SCRIPT-VIRTUAIS
// Funções que podem ser sobrescritas pelo Godot em diferentes contextos
#[godot_api]
impl Animal {
	#[func(virtual)]
	fn get_num_of_legs(&self) -> i32 {
		4
	}
}

/*
Dessa forma, no GDScript, você poderá criar as duas classes novas - em seus respectivos arquivos - fazendo com que elas extendam a 
classe Animal e modifiquem suas funções script-virtuais:

-------------------------------------------------------------------------------------------------------------------------------------
# arquivo Fish.gd
extends Animal

# sobrescrevemos com um underline (_) antes para seguir a convenção do GDScript de nomeação de funções virtuais
func _get_num_of_legs() -> int:
	return 0

-------------------------------------------------------------------------------------------------------------------------------------

-------------------------------------------------------------------------------------------------------------------------------------
# arquivo Spider.gd
extends Animal

func _get_num_of_legs() -> int:
	return 8

-------------------------------------------------------------------------------------------------------------------------------------

*/
/*
Se chamarmos agora get_num_of_legs() em uma variável do tipo Gd<Animal> no Rust, como abaixo, 3 coisas podem acontecer:

    Se a instância de Animal não possuir um script acossiado no Godot, então a função retornará 4;
    Se a instância de Animal não possuir um script Fish.gd acossiado, então a função retornará 0;
    Se a instância de Animal não possuir um script Spider.gd acossiado, então a função retornará 8;
*/
fn can_run(monster: Gd<Monster>) -> bool {
    // Pega o número de pernas
    let num_of_legs: i32 = monster.bind().get_num_of_legs();

    // Animais com mais de 1 perna conseguem correr (eu acho?)
    if (num_of_legs > 1) {
    	true
    }
    // Os outros não
    false
}
/*
Limitações
    Não é possível acessar a implementação padrão da função script-virtual através do GDScript; só nosso código em rust enxerga a 
    implementação padrão.
    Se no Rust, você possui uma referência para o objeto, mutável ou não (&mut self ou &self), e então chama um método script-virtual que 
    modifica o mesmo objeto (através de um atributo #[var], por exemplo), isto pode causar um pânico por conta de um borrow-duplo.
*/