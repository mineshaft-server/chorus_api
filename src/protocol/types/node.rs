use crate::util::identifier::Identifier;

pub struct BrigDoubleProps {
  pub flags: i8,
  pub min: Option<f64>,
  pub max: Option<f64>,
}

pub struct BrigFloatProps {
  pub flags: i8,
  pub min: Option<f32>,
  pub max: Option<f32>,
}

pub struct BrigIntegerProps {
  pub flags: i8,
  pub min: Option<i32>,
  pub max: Option<i32>,
}

pub enum BrigStringProps {
  SingleWord = 0,
  QuotablePhrase = 1,
  GreedyPhrase = 2,
}

pub enum Properties {
  BrigDouble(BrigDoubleProps),
  BrigFloat(BrigFloatProps),
  BrigInteger(BrigIntegerProps),
  BrigString(BrigStringProps),
  Entity(u8),
  ScoreHolder(bool),
  Range(bool),
}

pub struct Node {
  pub flags: i8,
  pub children: Vec<i32>,
  pub redirect_node: Option<i32>,
  pub name: Option<String>,
  pub parser: Option<Identifier>,
  pub properties: Option<Properties>,
  pub suggestion_type: Option<Identifier>,
}