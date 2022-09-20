
use core::panic;
use mysql::{params, Params};
use simple_logger::SimpleLogger;
pub mod rooster;
use rooster::{query::{SQLZygote}, connection::{ConnectionData}, Rooster, base_entity::{*}};


RoosterEntity!(
    pub struct TestSub { 
        id: u32,
        sub_name: String
    }
);


RoosterEntity! {
    pub struct Test { 
        id: u32,
        name: String,
        sub : TestSub
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let conn_data = ConnectionData::new(
        String::from("pnsv"),
        String::from("pnsvpnsv"),
        None,
        None
    );


    let mut rooster = Rooster::new(conn_data, None);
    rooster.connect();

    let values: Vec<Box<Test>> = rooster.get("Test").execute().unwrap(); 
    let params: Params = params! { 
        
    };


    for mut val in values { 
        println!("{:?}", val);

        let obj: &mut Test = val.as_mut();
        obj.name += "12";
        obj.id = 4 + obj.id;

        rooster.save(Box::new(obj));
    }

}
