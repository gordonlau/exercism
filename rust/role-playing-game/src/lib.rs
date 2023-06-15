// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        let new_player = Player {health: 0, mana: None , level: self.level};
        if self.health == 0 && self.level >= 10{
            Some(Player {health: 100, mana: Some(100), level: self.level})
        }else if self.health == 0{
            Some(Player {health: 100, mana: None, level: self.level})
        }else{
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health = match self.health > mana_cost{
                    true => self.health - mana_cost,
                    false => 0
                };
                0 
            },
            Some(mana) if mana < mana_cost => 0 ,
            Some(mana) => {
                self.mana = Some(mana -  mana_cost);
                mana_cost * 2
            }
        }
    }
}
