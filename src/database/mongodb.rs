///
///  you must know the Bson to use this code
/// 

use futures::TryStreamExt;
use mongodb::{
    Client, options::{ ClientOptions, FindOptions, TlsOptions, Tls },
    error::Result, Database, Collection, bson::Document
};
use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[derive(Debug, Clone)]
pub struct MongoDB {
    client: Client,
    db: Database
}

impl MongoDB {
    pub async fn new(cfg: MongoCfg) -> Result<Self>{
        let url = parse_url(& cfg);
        // let url = cfg.app_name;
        let mut client_options = ClientOptions::parse(url).await?;
        client_options.app_name = cfg.app_name;
        if let Some(tls) = cfg.tls {
            client_options.tls = Some(Tls::Enabled(tls));
        }
        let client = Client::with_options(client_options)?;
        let db = client.database(&cfg.database);
        Ok(Self {client, db})        
    }

    /// if you want to change database just nee to run this once
    pub fn set_db(&mut self, name: &str) -> &mut Self {
        let db = self.client.database(name);
        self.db = db;        
        self
    }

    fn get_collection<T>(&self, name: &str) -> Collection<T> {        
        self.db.collection::<T>(name)        
    }

    pub async fn find<T>(
        &self, collection_name: &str,
        filter: Option<Document>,
        find_option: Option<Document>,
        result: &mut Vec<T>
    )  -> Result<()>       
    where T: DeserializeOwned + Unpin + Send + Sync {
        let c = &self.get_collection::<T>(collection_name);
        let options = build_options(find_option);
        let mut cursor = c.find(filter, options).await?;

        while let Some(item) = cursor.try_next().await? {
            result.push(item);
        }
    
        Ok(())
    }

    pub async fn find_one<T>(&self, collection_name: &str, filter: Document) -> Result<Option<T>>
    where T: DeserializeOwned + Unpin + Send + Sync {
        let c = &self.get_collection::<T>(collection_name);        
        c.find_one(filter, None).await
    }

    pub async fn insert_many<T:Serialize>(&self, collection_name: &str, docs: Vec<T>) 
    -> Result<mongodb::results::InsertManyResult> {
        let c = &self.get_collection::<T>(collection_name);
        c.insert_many(docs, None).await
    }
    
    pub async fn insert_one<T:Serialize>(&self, collection_name: &str, doc: T) 
    -> Result<mongodb::results::InsertOneResult> {
        let c = &self.get_collection::<T>(collection_name);
        c.insert_one(doc, None).await
    }
    
    pub async fn update_many<T:Serialize>(&self, collection_name: &str, query: Document, update: Document) 
    -> Result<mongodb::results::UpdateResult> {
        let c = &self.get_collection::<T>(collection_name);
        c.update_many(query, update, None).await
    }

    pub async fn update_one<T:Serialize>(&self, collection_name: &str, query: Document, update: Document) 
    -> Result<mongodb::results::UpdateResult> {
        let c = &self.get_collection::<T>(collection_name);
        c.update_one(query, update, None).await
    }

    pub async fn delete_many<T:Serialize>(&self, collection_name: &str, query: Document) 
    -> Result<mongodb::results::DeleteResult> {
        let c = &self.get_collection::<T>(collection_name);
        c.delete_many(query, None).await
    }

    pub async fn delete_one<T:Serialize>(&self, collection_name: &str, query: Document) 
    -> Result<mongodb::results::DeleteResult> {
        let c = &self.get_collection::<T>(collection_name);
        c.delete_one(query, None).await
    }

    pub async fn replace_one<T:Serialize>(&self, collection_name: &str, query: Document, replacement: T) 
    -> Result<mongodb::results::UpdateResult> {
        let c = &self.get_collection::<T>(collection_name);
        c.replace_one(query, replacement, None).await
    }

    pub async fn drop<T:Serialize>(&self, collection_name: &str) 
    -> Result<()> {
        let c = &self.get_collection::<T>(collection_name);
        c.drop(None).await
    }

    pub async fn count_documents<T:Serialize>(&self, collection_name: &str, filter: Document) 
    -> Result<u64> {
        let c = &self.get_collection::<T>(collection_name);
        c.count_documents(filter, None).await
    }
    

}

fn build_options(find_option: Option<Document>) -> FindOptions {
    FindOptions::builder().sort(find_option).build()
}

#[derive(Debug, Deserialize)]
pub struct MongoCfg {
    pub host: String,
    pub port: u16,    
    pub database: String,    
    pub user_name: Option<String>,
    pub password: Option<String>,    
    pub app_name: Option<String>,
    pub tls: Option<TlsOptions>,
}

fn parse_url(cfg: &MongoCfg) -> String{
    match (&cfg.user_name, &cfg.password) {        
        (Some(user_name), Some(password)) =>
            format!("mongodb://{}:{}@{}:{}", user_name, password, cfg.host, cfg.port),
        _ =>
            format!("mongodb://{}:{}", cfg.host, cfg.port),
    }   
}

