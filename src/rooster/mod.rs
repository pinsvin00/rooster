
use core::panic;
use std::vec;
use mysql::{*, prelude::*};

use self::{config::Config, query::{SQLZygote, Operation, SQLOperation}, base_entity::{EntityInterface, create_entity_from_row}};
use self::{query::Q};

pub mod connection;
pub mod query;
pub mod config;
pub mod base_entity;


pub struct Rooster {
    conn_data: connection::ConnectionData,
    connection_pool: Option<Pool>,
    config: config::RoosterConfig,
    queue: Vec<Operation>,
}


impl Rooster {
    pub fn new(conn_data: connection::ConnectionData, config: Option<config::RoosterConfig>) -> Rooster {
        return Rooster {
            conn_data,
            config: config.unwrap_or(config::RoosterConfig::default()),
            connection_pool: None,
            queue: vec![],
        }
    }
    pub fn connect(&mut self) {
        let url = self.conn_data.to_url();
        let urlstr = &url[..];
        let result = Pool::new(urlstr);
        self.connection_pool = match result {
            Ok(pool) => Some(pool), 
            Err(err) => panic!("ROOSTER : connection error occured\n {}", err),
        };
    }

    fn generate_sql_obj_template(&self, entity: Box< &mut dyn EntityInterface>) -> String { 
        let names = entity.field_names();
        
        let mut columns_template = String::new();
        let mut values_template  = String::new();
        for name in names {
            columns_template += name;
            columns_template += " ,";
    
            let b: String = format!("{} ,", entity.field_by_name(name).to_sql_str());
            values_template.push_str(&b);
        }
        columns_template.pop();
        values_template.pop();
    
    
        let columns_template = columns_template.replace(":", "");
        let raw_sql = format!("INSERT INTO {} ({}) VALUES ({})",
                            entity.class_name(), columns_template, values_template);
                            
        return raw_sql;
    }
}

impl SQLZygote for Rooster {

    fn get(&mut self, table_name: &str) -> &mut Self {
        self.queue.push(Operation {
            operation: SQLOperation::SELECT(String::from(table_name)),
            params: Params::Empty,
            query: None,
        });
        return self;
    }

    fn _where(&mut self, _params: Q) -> &mut Self {
        self.queue.push(Operation {
            operation: SQLOperation::WHERE,
            params: Params::Empty,
            query : Some(_params),
        });

        return self;
    }


    fn delete(&mut self, table_name: &str) -> &Self {
        self.queue.push(Operation {
            operation: SQLOperation::UPDATE,
            params: params! {
                "table_name" => table_name,
            },
            query : None,
        });
        return self;
    }

    fn update(&mut self, table_name: &str) -> &mut Self {
        self.queue.push(Operation {
            operation: SQLOperation::UPDATE,
            params: params! {
                "table_name" => table_name,
            },
            query: None,
        });
        return self;
    }

    fn save(&mut self, entity: Box<&mut dyn base_entity::EntityInterface> ) {                 
        let query = self.generate_sql_obj_template(entity);
        let query_cpy = Clone::clone(&query);

        let conn = self.connection_pool.as_ref().unwrap().get_conn();
        let mut conn = match conn { 
            Ok(conn) => conn,
            Err(e) => {
                log::error!("Cannot connect to mysql server!");
                panic!("{}", e);
            },
        };

        let stmt = conn.exec_drop(query , ());
        match stmt { 
            Ok(_) => {},
            Err(e) => panic!("Rooster save error: {}\n raw_query {}" , e , query_cpy),
        }

        log::info!("SUCCESSFULLY RAN : {}", query_cpy);
    }


    fn execute<T>(&mut self) -> Option<Vec<Box<T>>> where 
        T: EntityInterface + Default
    {
        let conn = self.connection_pool.as_ref().unwrap().get_conn();

        let mut conn = match conn { 
            Ok(conn) => conn,
            Err(e) => {
                println!("Rooster SQL Error : {}", e.to_string());
                return None;
            }
        };

        let mut raw = String::new();

        for (_, oper) in self.queue.iter().enumerate() { 
            match &oper.operation { 
                SQLOperation::SELECT(table_name) => {
                    let chunk = format!("SELECT * FROM {} ", table_name );
                    raw += &chunk;
                },
                SQLOperation::WHERE => {
                    let where_raw = oper.query.as_ref().unwrap().to_sql();
                    raw.push_str(" WHERE ");
                    raw.push_str(where_raw.as_str());
                    
                },
                _ => {
                    panic!("Invalid SQL Operation");
                }
            }
        }

        log::info!("Rooster running query with sql: {}", raw);

        let mut entities = Vec::new();
        conn.query_iter(raw).unwrap().for_each(|row| {
            let u_row = row.unwrap();
            let entity = create_entity_from_row(u_row);
            entities.push(entity);

        });
            
        return Some(entities);
    }


}

