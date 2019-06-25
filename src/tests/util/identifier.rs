use crate::utility::Identifier;

#[test]
pub fn new_with_str_type() {
  let namespace = "mineshaft";
  let id = "identifier";
  let identifier = Identifier::new(namespace, id);
  assert_eq!(namespace, identifier.namespace);
  assert_eq!(id, identifier.id);
}

#[test]
pub fn new_with_string_type() {
  let namespace = String::from("mineshaft");
  let id = String::from("identifier");
  let identifier = Identifier::new(&namespace, &id);
  assert_eq!(namespace, identifier.namespace);
  assert_eq!(id, identifier.id);
}

#[test]
pub fn from_with_string_type() {
  let namespace = String::from("mineshaft");
  let id = String::from("identifier");
  let string = std::format!("{}:{}", namespace, id);
  let identifier = Identifier::from(&string);
  assert_eq!(namespace, identifier.namespace);
  assert_eq!(id, identifier.id);
}

#[test]
pub fn from_without_namespace() {
  let id = "dirt";
  let identifier = Identifier::from(&id);
  assert_eq!(identifier.namespace, "minecraft");
  assert_eq!(identifier.id, id);
}

#[test]
pub fn identifier_equality() {
  let id1 = Identifier::new("minecraft", "zombie");
  let id2 = Identifier::new("minecraft", "zombie");
  assert!(id1 == id2);
}

#[test]
pub fn identifier_inequality() {
  let id1 = Identifier::new("minecraft", "villager");
  let id2 = Identifier::new("minecraft", "zombie");
  assert!(id1 == id2);
}