use dotenvy::var;
use openidconnect::{ClientId, IssuerUrl, RedirectUrl};

pub fn database_url() -> String {
    var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn host() -> String {
    var("HOST").unwrap_or("localhost".into())
}

pub fn port() -> u16 {
    var("PORT")
        .unwrap_or("8000".into())
        .parse()
        .expect("PORT must be an u16")
}

pub fn issuer_url() -> IssuerUrl {
    IssuerUrl::new(var("ISSUER_URL").expect("ISSUER_URL must be set"))
        .expect("ISSUER_URL must be an URL")
}

pub fn client_id() -> ClientId {
    ClientId::new(var("CLIENT_ID").expect("CLIENT_ID must be set"))
}

pub fn redirect_url() -> RedirectUrl {
    RedirectUrl::new(var("REDIRECT_URL").expect("REDIRECT_URL"))
        .expect("REDIRECT_URL must be an url")
}
