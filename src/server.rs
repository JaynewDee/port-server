pub mod connection {
    use actix_web::{
        middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
    };

    use actix_files as act_fs;

    struct Config<'a> {
        addr: &'a str,
        port: u16,
    }

    impl<'a> Config<'a> {
        const fn default() -> Self {
            Self {
                addr: "127.0.0.1",
                port: 8080,
            }
        }
    }
    pub async fn launch() -> std::io::Result<()> {
        const CONFIG: Config = Config::default();

        let (addr, port) = (CONFIG.addr, CONFIG.port);

        HttpServer::new(|| App::new())
            .bind((addr, port))?
            .run()
            .await
    }
}
