use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, setup_alloc};
use serde::{Deserialize, Serialize};


setup_alloc!();


//constants for invertabrates
#[near_bindgen]
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
pub enum InvertabratesConst{
    Invertabrate,
    Arthropods,
    Mollusks,
    Annelids,
    Platyhelminthes,
    Nematodes,
    Echinoderms,
    Poriferous,
    Cnidarias,
    None,
}

fn value_of_invert_enum(invert: &InvertabratesConst) -> String{
    match invert{
        InvertabratesConst::Invertabrate => "Invertabrate".to_string(),
        InvertabratesConst::Arthropods => "Arthropods".to_string(),
        InvertabratesConst::Mollusks => "Mollusks".to_string(),
        InvertabratesConst::Annelids => "Annelids".to_string(),
        InvertabratesConst::Platyhelminthes => "Platyhelminthes".to_string(),
        InvertabratesConst::Nematodes => "Nematodes".to_string(),
        InvertabratesConst::Echinoderms => "Echinoderms".to_string(),
        InvertabratesConst::Poriferous => "Poriferous".to_string(),
        InvertabratesConst::Cnidarias => "Cnidarias".to_string(),
        InvertabratesConst::None => "None".to_string(),
    }
}

//class invertabrate
#[near_bindgen]
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize,  Deserialize, Serialize)]
//#[derive(Debug, Clone)]
pub struct Invertabrate{
    name: String,
    phylum: InvertabratesConst,
    class: InvertabratesConst
}

//default values for invertabrates attributes
const DEFAULT_NAME: &str = "animal-name";
const DEFAULT_ATTRIBUTE: InvertabratesConst = InvertabratesConst::None;

//default invertabrate
impl Default for Invertabrate{
    fn default() -> Self{
        Self{name: DEFAULT_NAME.to_string(),
            phylum: DEFAULT_ATTRIBUTE,
            class: DEFAULT_ATTRIBUTE}
    }
}

#[near_bindgen]
impl Invertabrate{
    //setters
    //call functions
    pub fn set_invert_name(&mut self, name: String){
        self.name = name;
        println!("name set to: {:?}", self.name);
    }
    pub fn set_invert_phylum(&mut self, phylum: InvertabratesConst){
        self.phylum = phylum;
        println!("phylum set to: {:?}", self.phylum);
    }
    pub fn set_invert_class(&mut self, class: InvertabratesConst){
        self.class = class;
        println!("class set to: {:?}", self.class);
    }


    //getters
    //view functions
    pub fn get_invert_name(&self) -> String{
        self.name.to_string()
    }
    pub fn get_invert_phylum(&self) -> String{
        let invert = &self.phylum;
        value_of_invert_enum(invert)
    }
    pub fn get_invert_class(&self) -> String{
        let invert = &self.class;
        value_of_invert_enum(invert)
    }
    
}