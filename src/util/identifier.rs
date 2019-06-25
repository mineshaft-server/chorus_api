#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Identifier {
  namespace: String,
  id: String
}

impl Identifier {
  pub fn new(namespace: &String, id: &String) -> Identifier {
    return Identifier {
      namespace: namespace.clone(),
      id: id.clone(),
    }
  }

  pub fn from(string: &String) -> Identifier {
    let parts: Vec<&str> = string.split(":").collect();
    if parts.len() > 1 {
      return Identifier {
        namespace: String::from(parts[0]),
        id: String::from(parts[1]),
      }
    } else {
      return Identifier {
        namespace: String::from("minecraft"),
        id: String::from(parts[1]),
      }
    }
  }
}