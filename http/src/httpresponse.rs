use std::collections::HashMap;
use std::io::{Result, Write};

// HttpResponse
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    /**
     * Default
     * @return HttpResponse
     */
    fn default() -> Self {
        Self {
            version: "HTTP/1.1". into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> String {
        let res1 = res.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res1.version(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            &res.body.unwrap().len(),
            &res1.body()
        )
    }
}

impl<'a> HttpResponse<'a> {

    /**
     * Create a new HttpResponse
     * @param status_code
     * @param headers
     * @param body
     */
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    )-> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        };
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match status_code {
            "200" => "OK".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            "400" => "Bad Request".into(),
            _ => "Unknown Error".into(),
        };

        response.body = body;

        response
    }
    
    /**
     * Send response
     * @param write_stream
     */
    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string : String = String::from(res);
        let _ = write!(write_stream, "{}", response_string );

        Ok(())
    }
    /**
     * Getter for body
     */
    fn version(&self) -> &str {
        self.version
    }

    /**
     * Getter for status_code
     */
    fn status_code(&self) -> &str {
        self.status_code
    }
    /**
     * Getter for status_text
     */
    fn status_text(&self) -> &str {
        self.status_text
    }
    /**
     * Getter for headers
     */
    fn headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }
    /**
     * Getter for body
     */
    fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new (
            "200",
            None,
            Some("xxxx".into()),
        );
        let response_execpted = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type","text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(response_actual, response_execpted);
    }

        #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new (
            "404",
            None,
            Some("xxxx".into()),
        );
        let response_execpted = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type","text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(response_actual, response_execpted);
    }

    #[test]
    fn test_http_response_creation() {
        let response_execpted = HttpResponse {
            version: "HTTP/1.1" ,
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type","text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        let http_string: String = response_execpted.into();
        let actual_string: String = "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 4\r\n\r\nxxxx".into();
        assert_eq!(http_string, actual_string);
    }
}