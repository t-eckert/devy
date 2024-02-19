use super::Provider;

#[derive(Clone)]
pub struct LocalProvider {}

impl LocalProvider {
    pub fn new() -> Self {
        Self {}
    }
}
