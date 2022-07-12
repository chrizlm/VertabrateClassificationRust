use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use serde::{Deserialize, Serialize};




//constants for invertebrates
#[near_bindgen]
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
pub enum InvertebratesConst{
    Invertebrate,
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

fn value_of_invert_enum(invert: &InvertebratesConst) -> String{
    match invert{
        InvertebratesConst::Invertebrate => "Invertebrate".to_string(),
        InvertebratesConst::Arthropods => "Arthropods".to_string(),
        InvertebratesConst::Mollusks => "Mollusks".to_string(),
        InvertebratesConst::Annelids => "Annelids".to_string(),
        InvertebratesConst::Platyhelminthes => "Platyhelminthes".to_string(),
        InvertebratesConst::Nematodes => "Nematodes".to_string(),
        InvertebratesConst::Echinoderms => "Echinoderms".to_string(),
        InvertebratesConst::Poriferous => "Poriferous".to_string(),
        InvertebratesConst::Cnidarias => "Cnidarias".to_string(),
        InvertebratesConst::None => "None".to_string(),
    }
}

//class invertebrate
#[near_bindgen]
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize,  Deserialize, Serialize)]
//#[derive(Debug, Clone)]
pub struct Invertebrate{
    name: String,
    phylum: InvertebratesConst,
    class: InvertebratesConst
}

//default values for invertebrates attributes
const DEFAULT_NAME: &str = "animal-name";
const DEFAULT_ATTRIBUTE: InvertebratesConst = InvertebratesConst::None;

//default invertebrate
impl Default for Invertebrate{
    fn default() -> Self{
        Self{name: DEFAULT_NAME.to_string(),
            phylum: DEFAULT_ATTRIBUTE,
            class: DEFAULT_ATTRIBUTE}
    }
}

#[near_bindgen]
impl Invertebrate{
    //setters
    //call functions
    pub fn set_invert_name(&mut self, name: String){
        self.name = name;
        println!("name set to: {:?}", self.name);
    }
    pub fn set_invert_phylum(&mut self, phylum: InvertebratesConst){
        self.phylum = phylum;
        println!("phylum set to: {:?}", self.phylum);
    }
    pub fn set_invert_class(&mut self, class: InvertebratesConst){
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