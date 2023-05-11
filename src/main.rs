
use core::panic;
use simple_logger::SimpleLogger;
pub mod rooster;
use rooster::{connection::{ConnectionData}, Rooster, base_entity::{*}, query::*};

RoosterEntity!(
    pub struct TestSub { 
        FIELDS
        id: u32,
        sub_name: String;
        RELATIONS;
    }
);


RoosterEntity! {
    pub struct Test {
        FIELDS
        id: u32,
        name: String;
        RELATIONS
        subs: Relation { 
            join_on: String::from("id"),
            table: String::from("sus"),
        };
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();
    let conn_data = ConnectionData::new(
        String::from("michal"),
        String::from("michal"),
        None,
        None
    );

    let mut rooster = Rooster::new(conn_data, None);
    rooster.connect();


    let query  = eq("id", "2")
    .and(eq("name", " 'balls' ")
    .or(eq("bussy", " 'sussy' ").or(eq("name", "'cockerton'"))));
    
    let values: Vec<Box<Test>> = rooster.get("Test")._where(query).execute().unwrap(); 

}
