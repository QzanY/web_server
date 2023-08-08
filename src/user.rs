use crate::error::Error;
use serde::{Deserialize,Serialize};
use serde_json;
use std::{collections::HashMap, sync::{Arc,Mutex}};


#[derive(Clone,Debug)]
pub struct DataBase
{
    user_store: Arc<Mutex<HashMap<String,User>>>
}   

#[derive(Clone,Serialize,Debug,Deserialize)]
pub struct User
{
    name: String,
    password: String,
}

impl User
{
    pub fn name(&self) -> String
    {
        self.name.clone()
    }
}


impl DataBase
{
    pub async fn new() -> Result<DataBase,Error>
    {
        Ok(Self { user_store: Arc::default() })
    }

    pub async fn create_user(&self,new_user: User) -> Result<User,Error>
    {
        let mut store = self.user_store.lock().unwrap();

        let id = new_user.name.clone();
        if store.contains_key(&new_user.name)
        {
            return Err(Error::RegisterErrorUserExists);
        }

        store.insert(id, new_user.clone());

        Ok(new_user)
    }

    pub async fn delete_user(&self,name: String) -> Result<User,Error>
    {
        let mut store = self.user_store.lock().unwrap();

        if !(store.contains_key(&name))
        {
            return Err(Error::DeleteUserError);
        }
        // TODO: Implement a better solution than .clone()
        let user = store.get(name.as_str()).unwrap().clone();

        store.remove(&name);

        Ok(user)
    }

    pub async fn search_user(&self,username:&String) -> bool
    {
        let store = self.user_store.lock().unwrap();
        store.contains_key(username)
        
    }

    pub async fn check_password(&self,username:&String,password:&String) -> bool
    {
        let store = self.user_store.lock().unwrap();

        &store.get(username).unwrap().password == password
    }
}