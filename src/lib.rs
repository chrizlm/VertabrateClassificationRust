use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::attached_deposit;
use near_sdk::{near_bindgen, setup_alloc, env, AccountId, Promise, StorageUsage};
use near_sdk::collections::Vector;
use near_sdk::collections::LookupMap;
use near_sdk::serde_json::to_vec;
use serde::{Deserialize, Serialize};



use std::io;
use std::env as other_env;
mod vertabrate;
mod invertabrate;

use crate::vertabrate::Vertabrate;
use crate::invertabrate::Invertabrate;

//enum crates
use crate::vertabrate::VertabrateConst::Vertabrate as EnumVertabrate;
use crate::vertabrate::VertabrateConst::ColdBlooded;
use crate::vertabrate::VertabrateConst::WarmBlooded;
use crate::vertabrate::VertabrateConst::Mammals;
use crate::vertabrate::VertabrateConst::Birds;
use crate::vertabrate::VertabrateConst::Insect;
use crate::vertabrate::VertabrateConst::Fish;
use crate::vertabrate::VertabrateConst::Amphibians;
use crate::vertabrate::VertabrateConst::Reptile;
use crate::invertabrate::InvertabratesConst::Invertabrate as EnumInvertabrate;

setup_alloc!();


//wrap in struct
#[near_bindgen]
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Classification{
    store: LookupMap<AccountId, Vector<Vertabrate>>,
}



impl Default for Classification{
    fn default() -> Self {
        Self{ store: LookupMap::new(b"s".to_vec()) }
    }
}



#[near_bindgen]
impl Classification {

    pub fn storage_staking(initial_storage: StorageUsage){
        let attached_deposit_amount = env::attached_deposit();
        let signer = env::signer_account_id();
        let current_storage = env::storage_usage();
        if let Some(used_storage) = u64::checked_sub(current_storage, initial_storage) {
            let storage_unit_price = env::storage_byte_cost();
            if let Some(payable_storage_cost) = u128::checked_mul(storage_unit_price, used_storage.into()) {
                assert!(attached_deposit_amount >= payable_storage_cost);

                let surplus = attached_deposit_amount - payable_storage_cost;
                let excess = if surplus > 0 { surplus } else { 0 };

                if excess > 0 {
                    let mut promise = Promise::new(signer);
                    promise.transfer(excess);
                }
            }
        }

    }


    pub fn classification_template(initial_storage: StorageUsage, animal_name: String, backbone: String, constbodytemp: String, xtics: String) -> Vertabrate{

        //create default objects of animals
        let mut vert_animal_prop = Vertabrate::default();
        let mut invert_animal_prop = Invertabrate::default();


        //get the options
        let one = String::from("Yes");
        let two = String::from("No");
        let feathers = String::from("Feathers");
        let far = String::from("Far");
        let threeparts = String::from("Three body parts");
        let gills = String::from("Gills");
        let slimskin = String::from("Smooth slimy skin");
        let skinscale = String::from("Rough skin scales");


        //classify according to backbone
        if backbone == one {
            vert_animal_prop.set_name(animal_name);
            vert_animal_prop.set_phylum(EnumVertabrate);

            //classification according to body temperature
            if constbodytemp == one {
                vert_animal_prop.set_class(WarmBlooded);

                //other characteristics
                if xtics == feathers{
                    vert_animal_prop.set_subclass(Birds);
                } else if xtics == far{
                    vert_animal_prop.set_subclass(Mammals);
                }


                Classification::storage_staking(initial_storage);


            }else if constbodytemp == two {
                vert_animal_prop.set_class(ColdBlooded);
                if xtics == threeparts{
                    vert_animal_prop.set_subclass(Insect);
                } else if xtics == gills{
                    vert_animal_prop.set_subclass(Fish);
                } else if xtics == slimskin{
                    vert_animal_prop.set_subclass(Amphibians);
                } else if xtics == skinscale{
                    vert_animal_prop.set_subclass(Reptile);
                }


                Classification::storage_staking(initial_storage);

            }
        }
        else if backbone == two {
            invert_animal_prop.set_invert_name(animal_name);
            invert_animal_prop.set_invert_phylum(EnumInvertabrate);
            env::log_str("we only classify vertabrate here");


            Classification::storage_staking(initial_storage);


        }

        vert_animal_prop
    }

    
   

    //classification method ... classify animals according to backbone existance
    #[payable]
    pub fn classify_vert_animal(&mut self, animal_name: String, backbone: String, constbodytemp: String, xtics: String) -> Vertabrate{


        let initial_storage = env::storage_usage();
        let signer = env::signer_account_id();
        let attached_deposit_amount = env::attached_deposit();


        if let Some(mut animals_data_store) = self.store.get(&signer){

           let vert_animal = Classification::classification_template(initial_storage, animal_name, backbone, constbodytemp, xtics);
            let display_animal = vert_animal.clone();
            let initial_storage = env::storage_usage();
            animals_data_store.push(&vert_animal);


            Classification::storage_staking(initial_storage);

            display_animal




        }else {
            let mut animals_data_store: Vector<Vertabrate> = Vector::new(b"a");

            let vert_animal = Classification::classification_template(initial_storage, animal_name, backbone, constbodytemp, xtics);
            let display_animal = vert_animal.clone();
            let initial_storage = env::storage_usage();
            let signer = env::signer_account_id();
            animals_data_store.push(&vert_animal);
            self.store.insert(&signer, &animals_data_store);



            Classification::storage_staking(initial_storage);

            display_animal
        }
        
        }





pub fn display_classified_vert(&self) -> Vec<Vertabrate>{
    let signer = env::signer_account_id();
    if let Some(mut animals_data_store) = self.store.get(&signer){
        animals_data_store.to_vec()
    }else {
        let mut animals_data_store = Vector::new(b"a");
        let mut vert_animal_prop = Vertabrate::default();
        animals_data_store.push(&vert_animal_prop);
        animals_data_store.to_vec()
    }

}



pub fn remove_animal(&mut self) {
    let signer = env::signer_account_id();
    if let Some(mut animals_data_store) = self.store.get(&signer){
        animals_data_store.pop();
    }
}


pub fn check_account(&mut self) -> AccountId {
    env::signer_account_id()
}

    

pub fn remove_all_animals(&mut self) {
    let signer = env::signer_account_id();
    if let Some(mut animals_data_store) = self.store.get(&signer){
        animals_data_store.clear();
    }
}


    //classify invertabrates
    pub fn classify_invertabrates(&self, mut invert_animal_prop: Invertabrate) -> Invertabrate{
        
        //let mut animals_data_store = self.store;

        //array of invertabrate classes
        let invert_array = [
            "Arthropods",
            "Mollusks",
            "Annelids",
            "Platyhelminthes",
            "Nematodes",
            "Echinoderms",
            "Poriferous",
            "Cnidarias",
        ];
        
        env::log_str("the animal is not a vertabrate but is in one of the following classes of invertabrates");

        for class in 0..invert_array.len() {
            near_sdk::log!("{}", invert_array[class]);
        }

        

        invert_animal_prop

    }



}



#[cfg(test)]
mod tests {

    use crate::{vertabrate::{Vertabrate, VertabrateConst}, invertabrate::{Invertabrate, InvertabratesConst}, Classification};


    #[test]
    fn test_vert_animal_default_properties(){
        let vert_animal = Vertabrate::default();
        assert_eq!(vert_animal.get_name(), "animal-name".to_string());
        assert_eq!(vert_animal.get_phylum(), "None".to_string());
        assert_eq!(vert_animal.get_class(), "None".to_string());
        assert_eq!(vert_animal.get_subclass(), "None".to_string());
    }


    #[test]
    fn test_vert_animal_properties(){
        let mut vert_animal = Vertabrate::default();

        let name = "vert_animal".to_string();
        let phylum = VertabrateConst::Vertabrate;
        let class = VertabrateConst::WarmBlooded;
        let subclass = VertabrateConst::Mammals;


        vert_animal.set_name(name);
        vert_animal.set_phylum(phylum);
        vert_animal.set_class(class);
        vert_animal.set_subclass(subclass);

        assert_eq!(vert_animal.get_name(), "vert_animal".to_string());
        assert_eq!(vert_animal.get_phylum(), "Vertabrate".to_string());
        assert_eq!(vert_animal.get_class(), "WarmBlooded".to_string());
        assert_eq!(vert_animal.get_subclass(), "Mammals".to_string());
    }

    #[test]
    fn test_invert_animal_default_properties(){
        let invert_animal = Invertabrate::default();
        assert_eq!(invert_animal.get_invert_name(), "animal-name".to_string());
        assert_eq!(invert_animal.get_invert_phylum(), "None".to_string());
        assert_eq!(invert_animal.get_invert_class(), "None".to_string());

    }

    #[test]
    fn test_invert_animal_properties(){
        let mut invert_animal = Invertabrate::default();

        let name = "invert_animal".to_string();
        let phylum = InvertabratesConst::Invertabrate;
        let class = InvertabratesConst::Arthropods;

        invert_animal.set_invert_name(name);
        invert_animal.set_invert_phylum(phylum);
        invert_animal.set_invert_class(class);

        assert_eq!(invert_animal.get_invert_name(), "invert_animal".to_string());
        assert_eq!(invert_animal.get_invert_phylum(), "Invertabrate".to_string());
        assert_eq!(invert_animal.get_invert_class(), "Arthropods".to_string());
    }

  /* 
    //following test individually
    //#[test]
    fn test_vert(){
        let test_obj = Vertabrate::default();
        let test_name = test_obj.get_name();
        let test_db: Vec<Vertabrate> = vec![];
        let class_test_obj = Classification::classify_vertabrates(test_obj, test_db);

        assert_eq!(class_test_obj.get_name(), test_name);
    }

    //#[test]
    fn test_invert(){
        let test_obj = Invertabrate::default();
        let test_name = test_obj.get_invert_name();
        let test_db: Vec<Vertabrate> = vec![];
        let class_test_obj = Classification::classify_invertabrates(test_obj, test_db);

        assert_eq!(class_test_obj.get_invert_name(), test_name);
    }

    //#[test]
    fn test_warm_blooded(){
        let mut test_obj = Vertabrate::default();
        test_obj.set_class(VertabrateConst::ColdBlooded);
        let test_class = test_obj.get_class();
        let test_db: Vec<Vertabrate> = vec![];
        let warm_test_obj = Classification::classify_warm_blooded(test_obj, test_db);

        assert_eq!(warm_test_obj.get_class(), test_class);
    }

    //#[test]
    fn test_cold_blooded(){
        let mut test_obj = Vertabrate::default();
        test_obj.set_class(VertabrateConst::ColdBlooded);
        let test_class = test_obj.get_class();
        let test_db: Vec<Vertabrate> = vec![];
        let cold_test_obj = Classification::classify_cold_bloodeed(test_obj, test_db);

        assert_eq!(cold_test_obj.get_class(), test_class);
    }

    //#[test]
    fn test_another_vert(){
        let mut test_obj = Vertabrate::default();
        test_obj.set_subclass(VertabrateConst::Mammals);
        let test_subclass = test_obj.get_subclass();
        let test_db: Vec<Vertabrate> = vec![];
        let another_vert = Classification::classify_another_vert(test_obj, test_db);

        assert_eq!(another_vert[0].get_subclass(), test_subclass);
    }

    //#[test]
    fn test_another_invert(){
        let mut test_db: Vec<Vertabrate> = vec![];
        let test_db_duplicate = test_db.clone();
        let another_invert = Classification::classify_another_invert(test_db);

        assert_eq!(another_invert.len(), test_db_duplicate.len());
    }
    
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

 */

}
