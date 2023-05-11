# rooster

# Avaible commands
```
insert(RoosterEntity)
save(RoosterEntity)
get(table_name)
_where(Query) -> QueryObject

eq(field_name, value) -> QueryObject //must be after the eq(), or(), and(), or where, note that when you want to input to method string value, value must be within "\'" mark or "\"" mark
or(Q) -> QueryObject
and(Q) -> QueryObject

```

# Getting started

## Build
Clone the repo, and simply start with cargo run.


## SQL part
Let's assume table with given values (first row are column names)
'id', 'name', 'description'
'2', 'aaa', 'testing'
'3', 'second name', 'uag'
'4', 'ggg', 'eee'

For each table that we want to use we should create a class that will have same names as columns
```
RoosterEntity! {
    pub struct Test {
        FIELDS
        id: u32,
        name: String,
        description: String;
        RELATIONS;
    }
}
```
RoosterEntity is a macro that prepares given class, for being mapped from mysql data. There are two mandatory sections, FIELDS that will contain all columns of table, fields in section should be comma separated, the last field must be ended with ";".
At the moment RELATIONS does absolutely nothing, but is still neccessary for macro to create a Rooster class.

## Code
The code below is a simple example of using rooster query builder. 
```
RoosterEntity! {
    pub struct Test {
        FIELDS
        id: u32,
        name: String,
        description: String;
        RELATIONS;
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();


    let conn_data = ConnectionData::new(
        String::from("michal"),
        String::from("michal"),
        None, //none if localhost 
        None  //none if port is 3306
    );


    let mut rooster = Rooster::new(conn_data, None);
    rooster.connect();

    //When using eq, be sure to use "\'" when typing string literal
    let query  = eq("name", "'aaa'").or(eq("id", "3"));
    
    let result = rooster.get("Test")._where(query).execute();
    let mut values : Vec<Box<Test>>;
    match result {
        Ok(entities) => {
            values = entities;
        }
        Err(message) => {
            log::info!("{}",message);
            return;
        }
    }

    if values.len() == 2
    {
        {
            let mut first_value = values[0].as_mut();
            first_value.name = "abde".into();
            first_value.description = "This is my first value description".into();

            let res = rooster.save(Box::new(first_value));
            if res.is_err() {
                log::info!("Failed to save first entity.");
            }
        }

        {
            let mut second_value = values[1].as_mut();

            second_value.name = "second name".into();
            second_value.description = "uag".into();

            let res = rooster.save(Box::new(second_value));
            if res.is_err() {
                log::info!("Failed to save second entity.");
            }
        }
    }

    let mut inserted_test = Test {
        id : 5,
        name : String::from("Some new name"),
        description : String::from("Some new description"),
    };
    let insert_result = rooster.insert(Box::new(&mut inserted_test));

    if insert_result.is_err() {
        log::info!("Failed to insert new entity.");
    }


}
```

# What will be done in the future?
- Unifying insert, and save method for convinience of user.
- Fetching related objects (1,n), (n,n), (1,1)
- Possibility to modify entity id.
