struct Developer {
  name: String,
  username: String,
  age: u8,
  location: String,
  web: String,
}

impl Developer {
  fn new(
    name: &str, 
    username: &str, 
    age: u8, 
    location: &str, 
    web: &str) -> Developer {
    Developer {
      name: name.to_string(),
      username: username.to_string(),
      age,
      location: location.to_string(),
      web: web.to_string(),
    }
  }
}

fn main() {
  let developer = Developer::new(
        "Rafael Leal Machado", 
        "dev-rafaelmachado", 
        20, 
        "Curitiba, PR - Brazil", 
        "https://dev-rafaelmachado.dev"
    );
}