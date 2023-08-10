use std::task::{Context, Poll};

use tower::Service;

#[derive(Debug, Clone)]
struct Token<'a> {
    access_token: &'a str,
    refresh_token: &'a str,
}

trait Handler {

}

#[derive(Debug, Clone)]
struct TokenRenewer<S, T: Handler + Clone> {
    inner : S,
    handler: T,
}



impl<S: Clone, T: Handler + Clone, Request> Service<Request> for TokenRenewer<S, T>
where
    S: Service<Request>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let mut this = self.clone();

        this.inner.call(req)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tower::ServiceExt;

    #[derive(Debug, Clone)]
    struct TestHandler;

    impl Handler for TestHandler {}

    #[tokio::test]
    async fn test_with_hyper(){
        let handler = TestHandler;
        let service = TokenRenewer {
            inner: hyper::Client::new(),
            handler,
        };

        let req = hyper::Request::builder()
            .uri("http://www.rust-lang.org/")
            .body(hyper::Body::empty())
            .unwrap();

        let res = service.oneshot(req).await.unwrap();
        assert_eq!(res.status(), hyper::StatusCode::MOVED_PERMANENTLY);
    }
}