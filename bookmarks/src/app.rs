use warp::Filter;

pub struct App;

impl App {
    pub fn new() -> Self {
        App
    }

    pub async fn run(&self) {
        let hello = warp::path!("hello" / String)
            .map(|name| format!("Hello, {}!", name));

        warp::serve(hello)
            .run(([127, 0, 0, 1], 3030))
            .await;
    }
}