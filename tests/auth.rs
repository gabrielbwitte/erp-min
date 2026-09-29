#[cfg(test)]
mod tests {
    use actix_web::{App, test};
    use erp_min::dto::auth::LoginRequest;
    use erp_min::routes::configure_routes;

    #[actix_web::test]
    async fn test_login_200() {
        let app = test::init_service(App::new().configure(configure_routes)).await;
        let payload = LoginRequest {
            username: "Usuário".to_string(),
            password: "Senha123".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/login")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
