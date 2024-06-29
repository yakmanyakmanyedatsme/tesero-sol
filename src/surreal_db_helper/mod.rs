use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealdb::opt::auth::{Scope,Jwt};
use surrealdb::engine::any;
use crate::customers::individuals::Individual;
use crate::general::credentials::Credentials;

pub async fn signup<C: surrealdb::Connection>(db: &Surreal<C>,  credentials: Credentials, scope_ts: String, np: String, dat_base: String) -> Result<Jwt, surrealdb::Error> {
    let jwt = db.signup(Scope {
        namespace: &np,
        database: &dat_base,
        scope: &scope_ts,
        params: credentials,
}).await?;
    Ok(jwt)
}

pub async fn signin<C: surrealdb::Connection>(db: &Surreal<C>, credentials: Credentials, scope_ts: String, np: String, dat_base: String) -> Result<(), surrealdb::Error> {
    db.signin(Scope {
        namespace: &np,
        database: &dat_base,
        scope: &scope_ts,
         params: credentials,
    }).await?;
    
    Ok(())
}

pub async fn connect_and_signup_db(
    db_url: &str,
    username: &str,
    password:&str,
    ind: Individual,
) -> Result<String , surrealdb::Error> {
    // Connect to the database
    let db = any::connect(db_url).await?;
    // Sign in as the root use
    // Use the specified namespace and database
        // Define a scope for authentication
    // Sign in as the root user
    db.signin(Root {
        username,
        password,
    }).await?;
    println!("->> DB connected in memory");
    db.use_ns("customers".to_string()).use_db("individuals".to_string()).await?;
    let jwt = signup(&db, ind.cred, "account".to_string(), "customers".to_string(), "individuals".to_string()).await?.into_insecure_token();
    Ok(jwt)
}

pub async fn connect_and_signin_db(
    db_url: &str,
    username: &str,
    password:&str,
    cred: Credentials,
) -> Result< () , surrealdb::Error> {
    // Connect to the database
    let db = any::connect(db_url).await?;
    // Sign in as the root use
    // Use the specified namespace and database
        // Define a scope for authentication
    // Sign in as the root user
    db.signin(Root {
        username,
        password,
    }).await?;
    println!("->> DB connected in memory");
    db.use_ns("customers".to_string()).use_db("individuals".to_string()).await?;
    let _jwt = signin(&db, cred, "account".to_string(), "customers".to_string(), "individuals".to_string()).await?;
    Ok(())
}