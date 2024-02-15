pub struct Router {}


impl Router {
    pub fn new() -> Self {
        Self{}
    }

    pub async fn serve(self) {
        println!("Serving!");
    }
}
