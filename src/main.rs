#![allow(unused)]

#[macro_use] extern crate serde;
#[macro_use] extern crate rocket;

use diesel_async::{
    pooled_connection::{
        deadpool::{BuildError, Pool}, AsyncDieselConnectionManager, PoolError
    },
    AsyncConnection, 
    AsyncPgConnection,
    RunQueryDsl
};
use gateway::SubscriptionState;

use core::panic;
use std::{env, error::Error};

mod channels;
pub use channels::Channel;

mod guilds;
pub use guilds::Guild;

mod messages;
pub use messages::Message;

mod users;
pub use users::{User, UserMetadata};

mod roles;
pub use roles::Role;

type Db = AsyncDieselConnectionManager<AsyncPgConnection>;
type DbPool = Pool<AsyncPgConnection>;

mod gateway;

pub mod schema; 

fn establish_db_connection() -> Result<DbPool, BuildError> {
    dotenvy::dotenv().ok();
    
    let config = Db::new(
        std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| panic!("DATABASE_URL env var must be set"))
    );

    Pool::builder(config).build()
}


#[launch]
async fn rocket() -> _ {
    let mut pool = match establish_db_connection() {
        Ok(p) => p,
        Err(err) => {
            panic!("Failed to connect to the database: {err}");
        }
    };

    if let Err(err) = users::setup(&pool).await {
        panic!("Error setting up the overlord {err}");
    };

    let mut subscriptions = SubscriptionState::new();

    rocket::build()
        .manage(pool)
        .manage(subscriptions)
        .mount("/users/", users::routes())
        .mount("/gateway/", gateway::routes())
}
