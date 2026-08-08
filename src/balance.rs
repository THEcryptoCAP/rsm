use std::collections::BTreeMap;

pub struct Pallet {
    balances : BTreeMap<String, u128>,

}

impl Pallet {
    // function used to instatntiate this pallet
    pub fn new() -> Self{
        Self{
            balances: BTreeMap::new()
        }
    }
    //setting balance of an account 
    pub fn set_balance(&mut self, who: &String, amount: u128){
        self.balances.insert(key: who.clone(), value: amount);
    }
    //fetching balance by name
    pub fn balance(&self, who: &String) -> u128{
        self.balances.get(key:who).unwrap_or(default: &0)
    }

}