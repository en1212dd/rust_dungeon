use crate::prelude::*;

const NUM_TILES:usize=(SCREEAN_WIDTH*SCREEAN_HEIGHT) as usize;

#[derive(Copy,Clone,PartialEq)]
pub enum TileType{
    Wall,
    Floor
}
pub struct Map{
    pub tiles:Vec<TileType>
}

impl Map {
    pub fn new() -> Self{
        Map{
            tiles: vec![TileType::Floor;NUM_TILES]
        }
    }
    pub fn map_indx(x:i32,y:i32)->usize{
        ((y*SCREEAN_WIDTH)+x) as usize
    }
    pub fn in_bounds( point:Point)->bool{
        point.x>=0&& point.x<SCREEAN_WIDTH
            && point.y>= 0 && point.y < SCREEAN_HEIGHT
    }
    pub fn can_enter_tile(&self, point:Point)->bool{
        Self::in_bounds(point) && self.tiles[Self::map_indx(point.x, point.y)]==TileType::Floor
    }
    pub fn try_idx(point:Point)->Option<usize>{
        if Self::in_bounds(point){
            None
        }else {
            Some(Self::map_indx(point.x, point.y))
        }
    }
    pub fn render(&self, ctx:&mut BTerm){
        for y in 0..SCREEAN_HEIGHT{
            for x in 0..SCREEAN_WIDTH{
                let idx = Self::map_indx(x,y);
                    match self.tiles[idx] {
                        TileType::Floor=>ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                        TileType::Wall=> ctx.set(x,y,GREEN, BLACK,to_cp437('#'))
                    }
            }
        }
    }
}