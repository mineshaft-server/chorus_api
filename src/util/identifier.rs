#[cfg(test)]
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Identifier {
  pub namespace: String,
  pub id: String
}

#[cfg(not(test))]
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Identifier {
  namespace: String,
  id: String
}

impl Identifier {
  pub fn new(namespace: &str, id: &str) -> Identifier {
    return Identifier {
      namespace: String::from(namespace),
      id: String::from(id),
    }
  }

  pub fn from(string: &str) -> Identifier {
    let temp = String::from(string);
    let parts: Vec<&str> = temp.split(":").collect();
    if parts.len() > 1 {
      return Identifier {
        namespace: String::from(parts[0]),
        id: String::from(parts[1]),
      }
    } else {
      return Identifier {
        namespace: String::from("minecraft"),
        id: String::from(parts[0]),
      }
    }
  }

  pub fn to_string(&self) -> String {
    return format!("{}:{}", self.namespace, self.id);
  }
}

impl Default for Identifier {
  fn default() -> Self { Identifier::new("", "") }
}