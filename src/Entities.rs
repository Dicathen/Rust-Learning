#![allow(non_snake_case)]
pub enum Type
{
    Compact,
    Sedan,
    SUV,
    Van,
    Other,   
}
pub struct Driver
{
    ID: i32,
    Name: String,
    VehicleCapacity: String,
    Handicap: bool,
    Rating: f32,
    Available: bool,
    Pets: bool,
    Notes: String,
    VehicleType: Type,
}

pub trait Print
{
    fn PrintInfo(&self);
}

impl Driver
{
    pub fn new(ID: i32, Name: String, VehicleCapacity: String, Handicap: bool, Rating: f32, Available: bool, Pets: bool, Notes: String, VehicleType: Type) -> Driver
    {
        Driver{
            ID,
            Name,
            VehicleCapacity,
            Handicap,
            Rating,
            Available,
            Pets,
            Notes,
            VehicleType,
        }
    }
}

impl Print for Driver
{
    fn PrintInfo(&self)
    {
        println!("Driver ID: {} \nDriver Name: {} \nDriver Capacity: {} \nDriver Handicap: {} \nDriver Rating: {} \nDriver Available: {} \nDriver Pets: {} \nDriver Notes: {} \nDriver Vehicle Type: {}", self.ID, self.Name, self.VehicleCapacity, self.Handicap, self.Rating, self.Available, self.Pets, self.Notes, self.VehicleType);
    }
}

impl std::fmt::Display for Type
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match *self
        {
            Type::Compact => write!(f, "Compact"),
            Type::Sedan => write!(f, "Sedan"),
            Type::SUV => write!(f, "SUV"),
            Type::Van => write!(f, "Van"),
            Type::Other => write!(f, "Other"),
        }
    }
}