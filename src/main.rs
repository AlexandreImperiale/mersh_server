extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate url;

fn main() {

    let mut mount = mount::Mount::new();
    let mut router = router::Router::new();

    // Redirection: '/' -> '/index.html'
    {
        let handler = |_: &mut iron::Request|
        {
            let base_url = url::Url::parse("http://localhost:8000").unwrap();
            let redirect_url = iron::Url::from_generic_url(base_url.join("/index.html").unwrap()).unwrap();
            Ok(iron::Response::with((iron::status::Found, iron::modifiers::Redirect(redirect_url))))
        };
        router.route(iron::method::Get, "/", handler, "redirect_to_index");
    }

    // Mounting router and static file services.
    mount
        .mount("/", router)
        .mount("/index.html", staticfile::Static::new(std::path::Path::new("src/front/index.html")))
        .mount("/static/bundle.js", staticfile::Static::new(std::path::Path::new("src/front/bundle.js")));

    // Creating iron http server.
    iron::Iron::new( mount)
        .http("localhost:8000")
        .unwrap();
}