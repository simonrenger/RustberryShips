#[derive(Clone, PartialEq, Debug)]
pub struct TankCmp{
    pub is_dead: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TilemapCmp{
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TilePosCmp{
    pub x: u32,
    pub y: u32,
}
