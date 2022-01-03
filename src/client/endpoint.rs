#[derive(Debug, Clone)]
pub struct Endpoint {
    ip: String,
}

impl Endpoint {
    pub fn new(ip: impl Into<String>) -> Endpoint {
        Endpoint { ip: ip.into() }
    }

    pub fn get(&self, path: impl Into<String>) -> String {
        format!("https://{}{}", self.ip, path.into())
    }
}
