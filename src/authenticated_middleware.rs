use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;

use actix_identity::Identity;
use actix_web::FromRequest;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
#[derive(Default)]
pub struct Authenticated;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Authenticated
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticatedMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticatedMiddleware { service }))
    }
}

pub struct AuthenticatedMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticatedMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let (http_req, _) = req.parts_mut();
        let identity = Identity::from_request(&http_req, &mut actix_http::Payload::None);

        let fut = self.service.call(req);

        Box::pin(async move {
            println!("Hi from response");
            let res = fut.await?;

            let identity = identity.await;
            println!("identity: {:#?}", identity.map(|i| i.identity()));

            Ok(res)
        })
    }
}
