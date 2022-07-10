use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, setup_alloc};
use serde::{Deserialize, Serialize};



use std::fmt::{Debug};
use std::string::ToString;

setup_alloc!();


#[near_bindgen]
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
pub enum VertabrateConst{
    Vertabrate,
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



fn value_of_vert_enum(vert: &VertabrateConst) -> String{
    match vert {
        VertabrateConst::Vertabrate => "Vertabrate".to_string(),
        VertabrateConst::WarmBlooded => "WarmBlooded".to_string(),
        VertabrateConst::ColdBlooded => "ColdBlooded".to_string(),
        VertabrateConst::Mammals => "Mammals".to_string(),
        VertabrateConst::Birds => "Birds".to_string(),
        VertabrateConst::Insect => "Insect".to_string(),
        VertabrateConst::Fish => "Fish".to_string(),
        VertabrateConst::Reptile => "Reptile".to_string(),
        VertabrateConst::Amphibians => "Amphibians".to_string(),
        VertabrateConst::None => "None".to_string(),
    }
}

//Class for Vertabrates
#[near_bindgen]
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Serialize)]
pub struct Vertabrate{
    name: String,
    phylum: VertabrateConst,
    class: VertabrateConst,
    subclass: VertabrateConst
}

//defaults for vertabrates
const DEFAULT_NAME: &str = "animal-name";
const DEFAULT_ATTRIBUTE: VertabrateConst = VertabrateConst::None;
//`core::option::Option`, `crate::vertabrate::VertabrateConst`, `std::option::Option`


//default values for vertabrates attributes
impl Default for Vertabrate{
    fn default() -> Self{
        Self{name: DEFAULT_NAME.to_string(),
            phylum: DEFAULT_ATTRIBUTE,
            class: DEFAULT_ATTRIBUTE,
            subclass: DEFAULT_ATTRIBUTE}
    }
}



//methods for the vertabrate class
#[near_bindgen]
impl Vertabrate{
    //setters
    //call functions
    pub fn set_name(&mut self, name: String){
        self.name = name;
        println!("name set to: {:?}", self.name);
    }
    pub fn set_phylum(&mut self, phylum: VertabrateConst){
        self.phylum = phylum;
        println!("phylum set to: {:?}", self.phylum);
    }
    pub fn set_class(&mut self, class: VertabrateConst){
        self.class = class;
        println!("class set to: {:?}", self.class);
    }
    pub fn set_subclass(&mut self, subclass: VertabrateConst){
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