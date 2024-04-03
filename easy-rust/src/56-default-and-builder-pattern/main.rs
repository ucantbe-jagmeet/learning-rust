// You can implement the Default trait to give values to a struct or enum that you think will be most common. The builder pattern works nicely with this to let users easily make changes when they want. First let's look at Default. Actually, most general types in Rust already have Default. They are not surprising: 0, "" (empty strings), false, etc.

#[derive(Debug)]
enum LifeState {
    Alive,
    Dead,
    NeverAlive,
    Uncertain,
}

#[derive(Debug)]
struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
}

impl Character {
    fn new(name: String, age: u8, height: u32, weight: u32, alive: bool) -> Self {
        Self {
            name,
            age,
            height,
            weight,
            lifestate: if alive {
                LifeState::Alive
            } else {
                LifeState::Dead
            },
        }
    }
    //Now comes the builder pattern. We will have many Billys, so we will keep the default. But a lot of other characters will be only a bit different. The builder pattern lets us chain very small methods to change one value each time. Here is one such method for Character:

    // Make sure to notice that it takes a mut self. We saw this once before, and it is not a mutable reference (&mut self). It takes ownership of Self and with mut it will be mutable, even if it wasn't mutable before. That's because .height() has full ownership and nobody else can touch it, so it is safe to be mutable. Then it just changes self.height and returns Self (which is Character).

    fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    fn weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }

    fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}
// But maybe in our world we want most of the characters to be named Billy, age 15, height 170, weight 70, and alive. We can implement Default so that we can just write Character::default(). It looks like this:
impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),
            age: 15,
            height: 170,
            weight: 70,
            lifestate: LifeState::Alive,
        }
    }
}

fn main() {
    // let character_1 = Character::new("Billy".to_string(), 15, 170, 70, true);
    // let character_1 = Character::default();
    let character_1 = Character::default().height(180).weight(60).name("Bobby");
    println!("{:#?}", character_1);
}
