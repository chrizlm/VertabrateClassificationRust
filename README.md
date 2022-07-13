# VertebrateClassificationRust

loom video link: https://www.loom.com/share/5f4863a303054149ba38c7728969f02d

About
This project is about classification of animals using their common features in particular vertebrates. It can help children to learn about classification by being able to classify animals around them such as their pets

Project
The project consists three files: lib, vertebrate and invertebrate

Lib contains the following:

i). Classification struct - it contains a store for the vertebrates that will be classified which is wrapped in a Lookupmap that contains account id as a parameter and a vector for storage
This will allow users be able to view their own storage

The struct has the following methods:

a). Storage staking - checks if user attached excess amount and refunds

b). Classification template - method that handles the classification by passing the traits of the animal and it will classify

c). Classify vert animal - this method is payable and is the main method for classification struct, it will check if signer had earlier stored classified animal if so the other animals classified will be added to the storage of signer if not a new storage will be created and signer account id attached

ii). Display classified vert - displays a list of all vertebrates that have been classified

iii). Check account - checks the account id of signer

iv). Remove animal - removes animals from list

v). Remove all animals - removes all the animals in the list

It all contains test classes for the functionality of the methods from the vertebrate and invertebrate files


Vertebrate

Contains an enum for things that are in the vertebrate category and vertebrate struct which contains getters and setters for the properties of the vertebrate animal

Invertebrate

Contains an enum for things that are in the invertebrate category and invertebrate struct which contains getters and setters for the properties of the vertebrate animal

Future works 

This project can be expanded to a bigger and broader classification of animals inclusive of both vertebrates and invertebrates to be able to help learners understand classification of animals better

OTHER FILES

It has a script folder containing bash scripts such as bash.sh and deploy.sh that have scripts for compiling and deploying the smart contract and call.sh that contain scripts for all call functions to the smart contract

How it works

run the build.sh file then the deploy.sh after that you can run the call.sh file and the scripts for the call function in the classification method will be executed
