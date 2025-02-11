#[tokio::main]
fn main() {
    let router = Router::new()
        .nest("/api", Router::new().route("/users", get(users_handler)))
        .nest("/admin", admin_router);
}

fn not_main() {
    let router = Router::new()
        .nest("/api", Router::new().route("/users", get(users_handler)))
        .nest("/admin", admin_router);
}
