struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        match self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            true => println!("{} is already naked...", self.name()),
            false => {
                println!("{} gets a haircut!", self.name);

                self.naked = true;
            }
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Self {
        Self { name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        match self.is_naked() {
            true => "baaaaah?",
            false => "baaaaah!",
        }
    }

    // Default trait methods can be overridden.
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // Type annotation is necessary in this case.
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
