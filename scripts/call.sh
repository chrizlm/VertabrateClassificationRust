#!/bin/bash

#note: you can change the account id to the one you have signed in with


#add items
near call test.chrislm.testnet classify_vert_animal '{"animal_name": "Cat", "backbone": "Yes", "constbodytemp": "Yes", "xtics": "Far"}' --accountId test.chrislm.testnet --amount 2
near call test.chrislm.testnet classify_vert_animal '{"animal_name": "Fish", "backbone": "Yes", "constbodytemp": "No", "xtics": "Gills"}' --accountId test.chrislm.testnet --amount 2
near call test.chrislm.testnet classify_vert_animal '{"animal_name": "Birdy", "backbone": "Yes", "constbodytemp": "Yes", "xtics": "Feathers"}' --accountId test.chrislm.testnet --amount 2
near call test.chrislm.testnet classify_vert_animal '{"animal_name": "Cat", "backbone": "No", "constbodytemp": "No", "xtics": "Three body parts"}' --accountId test.chrislm.testnet --amount 2
near call test.chrislm.testnet classify_vert_animal '{"animal_name": "Ant", "backbone": "Yes", "constbodytemp": "No", "xtics": "Three body parts"}' --accountId test.chrislm.testnet --amount 2
near call test.chrislm.testnet check_account  --accountId test.chrislm.testnet

#view list 
near call test.chrislm.testnet display_classified_vert --accountId test.chrislm.testnet

#remove an item
near call test.chrislm.testnet remove_animal  --accountId test.chrislm.testnet

#remove all items
#near call test.chrislm.testnet remove_all_animals  --accountId test.chrislm.testnet

#views the whole list
near call test.chrislm.testnet display_classified_vert --accountId test.chrislm.testnet
