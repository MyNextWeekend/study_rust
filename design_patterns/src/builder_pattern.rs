#[derive(Debug)]
pub struct Request {
    url: String,
    method: String, // should be enum
    headers: Vec<(String, String)>, // (name,value)
    body: Option<String>,
}


#[derive(Default, Clone)]
pub struct RequestBuilder {
    url: Option<String>,
    method: Option<String>,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        let _ = self.url.insert(url.into());
        self
    }
    pub fn method(mut self, method: impl Into<String>) -> Self {
        let _ = self.method.insert(method.into());
        self
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        let _ = self.body.insert(body.into());
        self
    }

    pub fn build(self) -> Result<Request, String> {
        let Some(url) = self.url else {
            return Err("No URL".to_string())
        };
        let method = self.method.unwrap_or_else(|| "GET".to_string());

        Ok(Request {
            url,
            method,
            headers: self.headers,
            body: self.body,
        })
    }
}


#[cfg(test)]
mod builder_test {
    use super::*;


    #[test]
    fn test_01() -> Result<(), String> {
        let builder = RequestBuilder::new().url("https://some-url.com/task");
        /// do something....

        let builder = builder.method("POST")
            .header("content-type", "application/json")
            .body("good body");
        /// do something....
        let req = builder.clone().build()?;
        println!("{req:#?}");

        // 一次构建多处使用
        let req = builder.header("ClientVersion", "1.2").build()?;
        println!("{req:#?}");
        Ok(())
    }
}