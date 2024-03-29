pub enum C2SPacketID {
  TeleportConfirm = 0x0,
  QueryBlockNBT = 0x1,
  ChatMessage = 0x2,
  ClientStatus = 0x3,
  ClientSettings = 0x4,
  TabComplete = 0x5,
  ConfirmTransaction = 0x6,
  EnchantItem = 0x7,
  ClickWindow = 0x8,
  CloseWindow = 0x9,
  PluginMessage = 0xA,
  EditBook = 0xB,
  QueryEntityNBT = 0xC,
  UseEntity = 0xD,
  KeepAlive = 0xE,
  Player = 0xF,
  PlayerPosition = 0x10,
  PlayerPositionAndLook = 0x11,
  PlayerLook = 0x12,
  VehicleMove = 0x13,
  SteerBoat = 0x14,
  PickItem = 0x15,
  CraftRecipeRequest = 0x16,
  PlayerAbilities = 0x17,
  PlayerDigging = 0x18,
  EntityAction = 0x19,
  SteerVehicle = 0x1A,
  RecipeBookData = 0x1B,
  NameItem = 0x1C,
  ResourcePackStatus = 0x1D,
  AdvancementTab = 0x1E,
  SelectTrade = 0x1F,
  SetBeaconEffect = 0x20,
  HeldItemChange = 0x21,
  UpdateCommandBlock = 0x22,
  UpdateCommandBlockMinecart = 0x23,
  CreativeInventoryAction = 0x24,
  UpdateStructureBlock = 0x25,
  UpdateSign = 0x26,
  Animation = 0x27,
  Spectate = 0x28,
  PlayerBlockPlacement = 0x29,
  UseItem = 0x2A,

}