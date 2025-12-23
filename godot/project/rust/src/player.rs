// Importa o que usaremos para implementar nosso jogador
use godot::prelude::*;
use godot::classes::Sprite2D;

// Declaramos o Player como sendo uma classe do Godot
// E fazemos ele "herdar" de Sprite2D. Ou seja, Player agora é um "nó filho" de Sprite2D, como você poderá ver em breve no editor.
#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64, 		 // Velocidade do jogador 
    angular_speed: f64,  // Velocidade angular do jogador

    base: Base<Sprite2D> // Nos permite acessar a classe base do jogador, neste caso, Sprite2D
}

// Importando a interface para os comportamentos de Sprite2D, incluindo a função init
// Cada classe tem um I{nome_da_classe} com suas funcionalidades
use godot::classes::ISprite2D;

// O #[godot_api] indica para o Godot que esta implementação é parte da API que estamos expondo para ele
#[godot_api]
impl ISprite2D for Player {
	// Declarando o constutor do Player com alguns valores padrão e uma mensagem no console do godot
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!");
        
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);

        let rotation = self.base().get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        self.base_mut().translate(velocity * delta as f32);
    }
}