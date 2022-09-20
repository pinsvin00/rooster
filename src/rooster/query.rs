use mysql::Params;
use super::base_entity::RoosterEntityInterface;



#[derive(Hash)]
#[derive(PartialEq, Eq)]
pub enum SQLOperation {
    //string has 
    SELECT(String),
    UPDATE,
    WHERE,
    DELETE,
    JOIN,
    ORDERBY
}

// static good_relations: HashMap<SQLOperation, Vec<SQLOperation> > = HashMap::from([
//     (SQLOperation::SELECT, vec![SQLOperation::WHERE, SQLOperation::JOIN]),
// ]);

pub struct Operation { 
    pub operation: SQLOperation,
    pub params : Params,
}


pub trait SQLZygote { 
    fn get(&mut self, table_name: &str) -> &mut Self;
    fn update(&mut self, table_name: &str) -> &mut Self;
    fn delete(&mut self, table_name: &str) -> &Self;
    fn _where(&mut self, conditions: Params) -> &mut Self;
    fn save(&mut self, entity: Box<&mut dyn RoosterEntityInterface>);
    //todo proper error handling
    fn execute<T>(&mut self) -> Option<Vec<Box<T>>> where
        T: RoosterEntityInterface + Default;
}