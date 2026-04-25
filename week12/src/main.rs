trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;
    fn age(&self) -> u8;
    
    fn describe(&self) -> String {
        format!("{} ({} years old) says: {}", self.name(), self.age(), self.sound())
    }
}

struct Dog {
    name: String,
    age: u8,
}

struct Cat {
    name: String,
    age: u8,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn sound(&self) -> &str {
        "Woof!"
    }
    
    fn age(&self) -> u8 {
        self.age
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn sound(&self) -> &str {
        "Meow!"
    }
    
    fn age(&self) -> u8 {
        self.age
    }
}

fn print_animal_sounds(animals: Vec<Box<dyn Animal>>) {
    for animal in animals {
        println!("{}", animal.describe());
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Buddy"),
        age: 5,
    };
    
    let cat = Cat {
        name: String::from("Whiskers"),
        age: 3,
    };
    
    println!("Dog: {}", dog.describe());
    println!("Cat: {}", cat.describe());
    
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(dog),
        Box::new(cat),
    ];
    
    print_animal_sounds(animals);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dog_sound() {
        let dog = Dog {
            name: String::from("Buddy"),
            age: 5,
        };
        assert_eq!(dog.sound(), "Woof!");
    }
    
    #[test]
    fn test_cat_sound() {
        let cat = Cat {
            name: String::from("Whiskers"),
            age: 3,
        };
        assert_eq!(cat.sound(), "Meow!");
    }
    
    #[test]
    fn test_dog_name() {
        let dog = Dog {
            name: String::from("Buddy"),
            age: 5,
        };
        assert_eq!(dog.name(), "Buddy");
    }
    
    #[test]
    fn test_dog_age() {
        let dog = Dog {
            name: String::from("Buddy"),
            age: 5,
        };
        assert_eq!(dog.age(), 5);
    }
    
    #[test]
    fn test_cat_describe() {
        let cat = Cat {
            name: String::from("Whiskers"),
            age: 3,
        };
        let description = cat.describe();
        assert!(description.contains("Whiskers"));
        assert!(description.contains("3 years old"));
        assert!(description.contains("Meow!"));
    }
}
