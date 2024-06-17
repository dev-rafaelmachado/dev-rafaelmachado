struct Bio {
    name: String,
    age: u8,
    location: String,
    languages: Vec<String>,
    skills: Vec<String>,
}

impl Bio {
    fn new() -> Bio {
        let name = String::from("Rafael");
        let age = 20;
        let location = String::from("Curitiba, PR - Brazil");
        let languages = vec![
            String::from("JavaScript"),
            String::from("TypeScript"),
            String::from("Rust"),
            String::from("Java"),
            String::from("C++"),
            String::from("Python"),
        ];
        let skills = vec![
            String::from("Frontend Development"),
            String::from("Backend Development"),
            String::from("Studying all the time"),
        ];

        println!("Hello, my name is {}", name);
        println!("I am {} years old", age);
        println!("I live in {}", location);
        println!("I am a developer and I have the following skills:");
        for skill in &skills {
            println!(" - {}", skill);
        }
        println!("I am also knowledgeable in the following languages:");
        for language in &languages {
            println!(" - {}", language);
        }

        Bio {
            name,
            age,
            location,
            languages,
            skills,
        }
    }
}