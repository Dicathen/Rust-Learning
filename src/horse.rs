#![allow(non_snake_case)]
pub struct Horse
{
    pub(crate) name: String,
    pub(crate) breed: String,
    pub(crate) colorDescription: String,
    pub(crate) otherComments: String,
    pub(crate) registrationID: String,
    pub(crate) spayedNeutered: String,
    pub(crate) weight: f64,
    pub(crate) gender: char,
}

impl Horse
{
    pub(crate) fn PrintInfo(&self)
    {
        println!("Name: {}",self.name);
        println!("Gender: {}",self.gender);
        println!("Breed: {}",self.breed);
        println!("Weight: {}",self.weight);
        println!("Spayed/Neutered: {}",self.spayedNeutered);
        println!("Registration ID: {}",self.registrationID);
        println!("Color Description: {}", self.colorDescription);
        println!("Other comments: {}",self.otherComments);
    }
}