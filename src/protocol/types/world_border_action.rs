pub struct WBALerpSize {
  pub old_diameter: f64,
  pub new_diameter: f64,
  pub speed: i64,
}

pub struct WBASetCenter {
  pub x: f64,
  pub z: f64,
}

pub struct WBAInit {
  pub x: f64,
  pub z: f64,
  pub old_diameter: f64,
  pub new_diameter: f64,
  pub speed: i64,
  pub portal_tp_boundary: i32,
  pub warning_time: i32,
  pub warning_blocks: i32,
}

pub enum WorldBorderAction {
  SetSize(f64),
  LerpSize(WBALerpSize),
  SetCenter(WBASetCenter),
  Initialize(WBAInit),
  SetWarnTime(i32),
  SetWarnBlocks(i32),
}