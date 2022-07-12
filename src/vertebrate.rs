use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use serde::{Deserialize, Serialize};



use std::fmt::Debug;
use std::string::ToString;



#[near_bindgen]
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
pub enum VertebrateConst{
    Vertebrate,
    WarmBlooded,
    ColdBlooded, 
    Mammals,
    Birds,
    Insect,
    Fish,
    Reptile,
    Amphibians,
    None
}



fn value_of_vert_enum(vert: &VertebrateConst) -> String{
    match vert {
        VertebrateConst::Vertebrate => "Vertebrate".to_string(),
        VertebrateConst::WarmBlooded => "WarmBlooded".to_string(),
        VertebrateConst::ColdBlooded => "ColdBlooded".to_string(),
        VertebrateConst::Mammals => "Mammals".to_string(),
        VertebrateConst::Birds => "Birds".to_string(),
        VertebrateConst::Insect => "Insect".to_string(),
        VertebrateConst::Fish => "Fish".to_string(),
        VertebrateConst::Reptile => "Reptile".to_string(),
        VertebrateConst::Amphibians => "Amphibians".to_string(),
        VertebrateConst::None => "None".to_string(),
    }
}

//Class for Vertebrates
#[near_bindgen]
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize)]
pub struct Vertebrate{
    name: String,
    phylum: VertebrateConst,
    class: VertebrateConst,
    subclass: VertebrateConst
}

//defaults for vertebrates
const DEFAULT_NAME: &str = "animal-name";
const DEFAULT_ATTRIBUTE: VertebrateConst = VertebrateConst::None;
//`core::option::Option`, `crate::vertebrate::VertebrateConst`, `std::option::Option`


//default values for vertebrates attributes
impl Default for Vertebrate{
    fn default() -> Self{
        Self{name: DEFAULT_NAME.to_string(),
            phylum: DEFAULT_ATTRIBUTE,
            class: DEFAULT_ATTRIBUTE,
            subclass: DEFAULT_ATTRIBUTE}
    }
}



//methods for the vertebrate class
#[near_bindgen]
impl Vertebrate{
    //setters
    //call functions
    pub fn set_name(&mut self, name: String){
        self.name = name;
        println!("name set to: {:?}", self.name);
    }
    pub fn set_phylum(&mut self, phylum: VertebrateConst){
        self.phylum = phylum;
        println!("phylum set to: {:?}", self.phylum);
    }
    pub fn set_class(&mut self, class: VertebrateConst){
        self.class = class;
        println!("class set to: {:?}", self.class);
    }
    pub fn set_subclass(&mut self, subclass: VertebrateConst){
        self.subclass = subclass;
        println!("subclass set to: {:?}", self.subclass);
    }

    //getters
    //view functions
    pub fn get_name(&self) -> String{
        self.name.to_string()
    }
    pub fn get_phylum(&self) -> String{
        let vert = &self.phylum;
        value_of_vert_enum(vert)
    }
    pub fn get_class(&self) -> String{
        let vert = &self.class;
        value_of_vert_enum(vert)
    }
    pub fn get_subclass(&self) -> String{
        let vert = &self.subclass;
        value_of_vert_enum(vert)
    }
}