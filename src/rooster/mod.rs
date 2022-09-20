
use core::panic;
use std::{collections::HashMap, vec, ops::{Deref, Add}, fmt::format};
use mysql::{*, prelude::*};

use self::{config::Config, query::{SQLZygote, Operation, SQLOperation}, base_entity::RoosterEntityInterface};


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
        let url = self.conn_data.toUrl();
        let urlstr = &url[..];
        let result = Pool::new(urlstr);
        self.connection_pool = match result {
            Ok(pool) => Some(pool), 
            Err(err) => panic!("ROOSTER : connection error occured\n {}", err),
        };
    }

    fn generate_sql_obj_template(&self, entity: Box< &mut dyn RoosterEntityInterface>) -> String { 
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
        });
        return self;
    }

    fn _where(&mut self, _params: Params) -> &mut Self {
        self.queue.push(Operation {
            operation: SQLOperation::WHERE,
            params: _params,
        });

        return self;
    }


    fn delete(&mut self, table_name: &str) -> &Self {
        self.queue.push(Operation {
            operation: SQLOperation::UPDATE,
            params: params! {
                "table_name" => table_name,
            },
        });
        return self;
    }

    fn update(&mut self, table_name: &str) -> &mut Self {
        self.queue.push(Operation {
            operation: SQLOperation::UPDATE,
            params: params! {
                "table_name" => table_name,
            },

        });
        return self;
    }

    fn save(&mut self, entity: Box<(&mut dyn base_entity::RoosterEntityInterface)> ) {                 
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
        T: RoosterEntityInterface + Default
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
                    let args = &oper.params;
                    let params = match args {
                        Params::Named(map) => map, 
                        Params::Positional(_) => todo!("todo uwu"), 
                        Params::Empty => todo!("todo uwu"),
                    };

                    let mut where_raw = String::new();
                    for (k,v) in params { 
                        let fragment = format!("{} = {} " , k, v.as_sql(true));
                        where_raw.push_str(fragment.as_str());
                    }
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
            let columns = u_row.columns_ref();


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
                value.load_from_row(&u_row, i);

                print!("\n");

            }
            entities.push(entity);

        });
            
        return Some(entities);
    }


}

