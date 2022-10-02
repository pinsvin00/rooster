use mysql::{Params, Value};
use super::base_entity::EntityInterface;


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
    pub query : Option<Q>,
}

#[derive(Clone, Copy)]
pub enum Conjunction{ 
    AND,
    OR,
    GROUP,
    START,
}

impl Conjunction {
    fn to_str(self) -> String { 
        match self {
            Conjunction::AND => {return String::from("AND")},
            Conjunction::OR => {return String::from("OR")},
            Conjunction::GROUP => return String::from(""),
            Conjunction::START => return String::from(""),
        }
    }
}

#[derive(Clone)]
pub enum Condition { 
    LIKE(String, String),
    EQUALS(String, String),
    GE(String, Value), //GREATER EQUAL
    G(String, Value),  //GREATER
    LE(String, Value), //LESS EQUAL
    L(String, Value),   //LESS
    NONE,
}



impl Condition { 
    pub fn as_sql(&self) -> String { 
        match &self { 
            Condition::LIKE(field_name, expr) => return format!("{} LIKE {}", field_name, expr),
            Condition::EQUALS(field_name, expr) => return format!("{} = {}", field_name, expr),
            Condition::GE(_, _) => todo!(),
            Condition::G(_, _) => todo!(),
            Condition::LE(_, _) => todo!(),
            Condition::L(_, _) => todo!(),
            Condition::NONE => return String::new()
        }
    }
}




//Query grouper
pub struct Q{ 
    next : Option<Box<Q>>,
    inner_next: Option<Box<Q>>,
    child_count: u32,
    is_group: bool,
    condition : Condition,
    conjunction : Conjunction,
}

impl Q {

    //converts group to the sql expression
    //I think that initial thing should be a group, just for easier recursive structure
    pub fn to_sql(&self) -> String {

        //lets assume it is group
        let pp = Box::new(self);
        let mut sql_raw = self.condition.as_sql();
        let con = &pp.conjunction.to_str();

        if self.is_group {
            match &pp.inner_next {
                Some(next ) => {
                    sql_raw += &format!("{}({})", con ,next.to_sql() ); 
                } 
                None => {}
            }
        }

        match &pp.next {
            Some(next) => {

                let b = format!(" {} {}", con , &next.to_sql());
                sql_raw += &b;
            }
            None => {}
        }

        return sql_raw;
    }

    pub fn set_tail(&mut self, q: Q) {
        self.child_count += 1;
        let mut pp = self;
        
        if pp.next.is_none() {
            pp.next = Some(Box::new(q));
            return;
        }
        while let Some(p) = pp.next.as_mut() {
            pp = p;
            //i cant belive that i have to do this because of this shitty language
            if pp.next.is_none() {
                pp.next = Some(Box::new(Q::shallow_copy(&q)));
                println!("Test!");
                return;
            }
        }
    }


    pub fn shallow_copy(q: &Q) -> Q {
        return Q {
            next : None,
            inner_next: None,
            child_count : 0,
            is_group: q.is_group,
            condition: q.condition.clone(),
            conjunction : q.conjunction,
        }
    }

    //creates a new logic group
    pub fn n() -> Q { 
        return Q {
            //should it be group or not? 
            is_group: false,
            inner_next: None,
            next: None,
            child_count : 0,
            conjunction: Conjunction::START,
            condition : Condition::LIKE(String::from("id"), String::from("1")),
        }
    }

    pub fn and (mut self, tail : Q) -> Q{

        if tail.child_count != 0 {
            self.set_tail(Q { 
                is_group: true,
                inner_next: Some(Box::new(tail)),
                next: None,
                condition : Condition::NONE,
                child_count: 1,
                conjunction: Conjunction::GROUP,
            })
        } else { 
            self.set_tail(tail);
        }

        self.conjunction = Conjunction::AND;
        self
    }
    
    pub fn or (mut self, tail : Q ) -> Q {
        
        if tail.child_count != 0 {
            self.set_tail(Q { 
                is_group: true,
                inner_next: Some(Box::new(tail)),
                next: None,
                condition : Condition::NONE,
                child_count: 1,
                conjunction: Conjunction::GROUP,
            })
        } else { 
            self.set_tail(tail);
        }

        self.conjunction = Conjunction::OR;
        self
    }

}


    pub fn like(field_name: String, like_str: String) -> Q{
        return Q { 
            is_group: false,
            inner_next: None,
            next : None,
            child_count : 0,
            conjunction: Conjunction::START,
            condition: Condition::LIKE(field_name, like_str),
        }
    }

    pub fn eq(field_name : &str, val : &str) -> Q { 
        return Q { 
            is_group: false,
            next : None,
            inner_next: None,
            child_count : 0,
            conjunction: Conjunction::START,
            condition: Condition::EQUALS(String::from(field_name), String::from(val)),
        }
    }

pub trait SQLZygote { 
    fn get(&mut self, table_name: &str) -> &mut Self;
    fn update(&mut self, table_name: &str) -> &mut Self;
    fn delete(&mut self, table_name: &str) -> &Self;
    fn _where(&mut self, conditions: Q) -> &mut Self;
    fn save(&mut self, entity: Box<&mut dyn EntityInterface>);
    //todo proper error handling
    fn execute<T>(&mut self) -> Option<Vec<Box<T>>> where
        T: EntityInterface + Default;
}

