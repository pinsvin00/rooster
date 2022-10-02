use core::panic;
use mysql::{*};

pub trait EntityInterface {
    fn field_names(&self) -> &'static [&'static str];
    fn field_by_name(&mut self, f_name: &str) -> Box<&mut dyn RoosterAtomicTypeInterface>;
    fn class_name(&self) -> String;
    fn get_relation_vec(&mut self, v_name: &str) -> Vec<Box<dyn EntityInterface>>;
    fn get_relations(&self) -> &'static [&'static str];
}


#[derive(Default)]
pub struct Relation { 
    container: Vec<Box<dyn EntityInterface>>,
    join_on: String,
    table: String,
}


pub fn create_entity_from_row<T> (row: Row) -> Box<T> 
where T: EntityInterface + Default { 
    let columns = row.columns_ref();

    let mut entity : Box<T> = Box::new(Default::default());

    let names = entity.field_names();

    for i in 0..columns.len() { 
        let column  = &columns[i];
        let column_name_str = column.name_str();
        let field_in_entity = names.iter().any(|&x| x == column_name_str);

        if !field_in_entity { 
            log::warn!("Omitted field {}", column_name_str);
            continue;
        }

        let value = entity.field_by_name(&column_name_str);

        value.load_from_row(&row, i);
        print!("\n");
    }

    return entity;
}


#[macro_export]
macro_rules! RoosterEntity {
    (pub struct $name:ident {
        FIELDS $($fname:ident : $ftype:ty ),+;
        RELATIONS  $($vname:ident : $vtype:ty ),*;
    }
    ) => {
        #[derive(Default)]
        pub struct $name {
            $(pub $fname : $ftype,)*
            $(pub $vname : $vtype,)*
        }

        impl EntityInterface for $name {
            fn field_names(&self) -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }


            fn field_by_name(&mut self, f_name: &str) -> Box<&mut dyn RoosterAtomicTypeInterface> { 
                match f_name { 
                    $( stringify!($fname) => return Box::new(&mut self.$fname), )*
                    _ => panic!("Panik!"),
                }
            }


            fn get_relations(&self) -> &'static [&'static str] { 
                static NAMES: &'static [&'static str] = &[$(stringify!($vname)),*];
                NAMES
            }

            fn get_relation_vec(&mut self, v_name: &str) -> Vec<Box<dyn EntityInterface>> {

                match v_name {
                    $ ( stringify!($vname) => return vec![],)*
                    _ => panic!("Panik!"),
                }
            }


            fn class_name(&self) -> String { 
                return String::from(stringify!($name));
            }

        }
    }
}


pub trait RoosterAtomicTypeInterface { 
    fn to_bytes(&mut self) -> Vec<u8>;
    fn from_bytes(&mut self, bytes : Vec<u8> );
    fn load_from_row(&mut self, row : &Row, idx: usize);
    fn to_sql_str(&mut self) -> String;
}




impl RoosterAtomicTypeInterface for String {
    fn to_bytes(&mut self) -> Vec<u8> {
        let bytes = self.as_bytes().to_vec();
        return bytes;
    }

    fn from_bytes(&mut self, bytes: Vec<u8>) {
        self.clear();
        for byte in bytes { 
            self.push(byte as char);
        }
    }

    fn load_from_row(&mut self, row : &Row, idx: usize) {
        let bytes: Option<Vec<u8>> = row.get(idx);
        self.from_bytes(bytes.unwrap());
    }

    fn to_sql_str(&mut self) -> String {
        return format!(" '{}' ", self);
    }

}

impl RoosterAtomicTypeInterface for u32 {
    fn to_bytes(&mut self) -> Vec<u8> {
        let bytes = self.to_be_bytes().to_vec();
        return bytes;
    }

    fn from_bytes(&mut self, bytes : Vec<u8> ) {
        if bytes.len() != 4 {
            panic!("Invalid u32 size, expected 4 bytes, received {} bytes!", bytes.len());
        }
        let a = u32::from_be_bytes([bytes[3], bytes[2], bytes[1], bytes[0]]);
        *self = a;
    }

    fn load_from_row(&mut self, row : &Row, idx: usize) {
        let new_val: u32 = row.get(idx).unwrap();
        *self = new_val
    }

    fn to_sql_str(&mut self) -> String {
        return format!(" '{}' ", self);
    }
}

