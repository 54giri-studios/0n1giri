use rocket::Route;

mod messages;
mod channels;

pub fn routes() -> Vec<Route> {
    let mut routes = routes![];

    routes.extend(routes![messages::get_channel_history]);
    routes.extend(routes![channels::get_channel]);
    routes.extend(routes![channels::subscribe]);

    routes
}
