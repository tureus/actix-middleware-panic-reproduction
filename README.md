Actix Middleware Panic Reproduction
---

In your first terminal:

    $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/actix-middleware-panic-reproduction`

In your second terminal:

    $ curl http://localhost:4000/
    curl: (52) Empty reply from server

and the first terminal will print out:

    Hi from start. You requested: /
    Hi from response
    thread 'actix-server worker 0' panicked at 'called `Option::unwrap()` on a `None` value', /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-web-4.1.0/src/request.rs:148:43
    stack backtrace:
    0: rust_begin_unwind
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/std/src/panicking.rs:584:5
    1: core::panicking::panic_fmt
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/core/src/panicking.rs:142:14
    2: core::panicking::panic
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/core/src/panicking.rs:48:5
    3: core::option::Option<T>::unwrap
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/core/src/option.rs:775:21
    4: actix_web::request::HttpRequest::match_info_mut
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-web-4.1.0/src/request.rs:148:14
    5: actix_web::service::ServiceRequest::match_info_mut
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-web-4.1.0/src/service.rs:242:9
    6: <actix_web::service::ServiceRequest as actix_router::resource_path::Resource>::resource_path
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-web-4.1.0/src/service.rs:332:9
    7: actix_router::resource::ResourceDef::capture_match_info_fn
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-router-0.5.0/src/resource.rs:685:20
    8: actix_router::router::Router<T,U>::recognize_fn
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-router-0.5.0/src/router.rs:61:16
    9: <actix_web::app_service::AppRouting as actix_service::Service<actix_web::service::ServiceRequest>>::call
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-web-4.1.0/src/app_service.rs:308:19
    10: <alloc::rc::Rc<S> as actix_service::Service<Req>>::call
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-service-2.0.2/src/lib.rs:227:9
    11: <actix_identity::middleware::IdentityServiceMiddleware<S,T> as actix_service::Service<actix_web::service::ServiceRequest>>::call::{{closure}}
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-identity-0.4.0/src/middleware.rs:100:35
    12: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/core/src/future/mod.rs:91:19
    13: <core::pin::Pin<P> as core::future::future::Future>::poll
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/core/src/future/future.rs:124:9
    14: <actix_middleware_panic_reproduction::authenticated_middleware::AuthenticatedMiddleware<S> as actix_service::Service<actix_web::service::ServiceRequest>>::call::{{closure}}
    at ./src/authenticated_middleware.rs:65:26
    15: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/core/src/future/mod.rs:91:19
    16: <core::pin::Pin<P> as core::future::future::Future>::poll
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/core/src/future/future.rs:124:9
    17: <actix_service::map_err::MapErrFuture<A,Req,F,E> as core::future::future::Future>::poll
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-service-2.0.2/src/map_err.rs:99:9
    18: actix_http::h1::dispatcher::InnerDispatcher<T,S,B,X,U>::handle_request
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-http-3.2.1/src/h1/dispatcher.rs:653:34
    19: actix_http::h1::dispatcher::InnerDispatcher<T,S,B,X,U>::poll_request
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-http-3.2.1/src/h1/dispatcher.rs:744:33
    20: <actix_http::h1::dispatcher::Dispatcher<T,S,B,X,U> as core::future::future::Future>::poll
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-http-3.2.1/src/h1/dispatcher.rs:1126:21
    21: <actix_http::service::HttpServiceHandlerResponse<T,S,B,X,U> as core::future::future::Future>::poll
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-http-3.2.1/src/service.rs:712:45
    22: <actix_service::and_then::AndThenServiceResponse<A,B,Req> as core::future::future::Future>::poll
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-service-2.0.2/src/and_then.rs:114:37
    23: <actix_server::service::StreamService<S,I> as actix_service::Service<(actix_server::worker::WorkerCounterGuard,actix_server::socket::MioStream)>>::call::{{closure}}
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-server-2.1.1/src/service.rs:75:30
    24: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/core/src/future/mod.rs:91:19
    25: tokio::runtime::task::core::CoreStage<T>::poll::{{closure}}
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/task/core.rs:165:17
    26: tokio::loom::std::unsafe_cell::UnsafeCell<T>::with_mut
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/loom/std/unsafe_cell.rs:14:9
    27: tokio::runtime::task::core::CoreStage<T>::poll
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/task/core.rs:155:13
    28: tokio::runtime::task::harness::poll_future::{{closure}}
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/task/harness.rs:480:19
    29: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/core/src/panic/unwind_safe.rs:271:9
    30: std::panicking::try::do_call
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/std/src/panicking.rs:492:40
    31: ___rust_try
    32: std::panicking::try
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/std/src/panicking.rs:456:19
    33: std::panic::catch_unwind
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/std/src/panic.rs:137:14
    34: tokio::runtime::task::harness::poll_future
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/task/harness.rs:468:18
    35: tokio::runtime::task::harness::Harness<T,S>::poll_inner
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/task/harness.rs:104:27
    36: tokio::runtime::task::harness::Harness<T,S>::poll
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/task/harness.rs:57:15
    37: tokio::runtime::task::raw::poll
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/task/raw.rs:144:5
    38: tokio::runtime::task::raw::RawTask::poll
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/task/raw.rs:84:18
    39: tokio::runtime::task::LocalNotified<S>::run
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/task/mod.rs:381:9
    40: tokio::task::local::LocalSet::tick::{{closure}}
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/task/local.rs:544:54
    41: tokio::coop::with_budget::{{closure}}
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/coop.rs:102:9
    42: std::thread::local::LocalKey<T>::try_with
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/std/src/thread/local.rs:445:16
    43: std::thread::local::LocalKey<T>::with
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/std/src/thread/local.rs:421:9
    44: tokio::coop::with_budget
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/coop.rs:95:5
    45: tokio::coop::budget
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/coop.rs:72:5
    46: tokio::task::local::LocalSet::tick
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/task/local.rs:544:31
    47: <tokio::task::local::RunUntil<T> as core::future::future::Future>::poll::{{closure}}
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/task/local.rs:761:16
    48: tokio::macros::scoped_tls::ScopedKey<T>::set
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/macros/scoped_tls.rs:61:9
    49: tokio::task::local::LocalSet::with
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/task/local.rs:582:9
    50: <tokio::task::local::RunUntil<T> as core::future::future::Future>::poll
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/task/local.rs:747:9
    51: tokio::task::local::LocalSet::run_until::{{closure}}
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/task/local.rs:502:18
    52: <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/core/src/future/mod.rs:91:19
    53: <core::pin::Pin<P> as core::future::future::Future>::poll
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/core/src/future/future.rs:124:9
    54: tokio::runtime::basic_scheduler::CoreGuard::block_on::{{closure}}::{{closure}}::{{closure}}
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/basic_scheduler.rs:543:48
    55: tokio::coop::with_budget::{{closure}}
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/coop.rs:102:9
    56: std::thread::local::LocalKey<T>::try_with
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/std/src/thread/local.rs:445:16
    57: std::thread::local::LocalKey<T>::with
    at /rustc/d68e7ebc38cb42b8b237392b28045edeec761503/library/std/src/thread/local.rs:421:9
    58: tokio::coop::with_budget
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/coop.rs:95:5
    59: tokio::coop::budget
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/coop.rs:72:5
    60: tokio::runtime::basic_scheduler::CoreGuard::block_on::{{closure}}::{{closure}}
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/basic_scheduler.rs:543:25
    61: tokio::runtime::basic_scheduler::Context::enter
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/basic_scheduler.rs:367:19
    62: tokio::runtime::basic_scheduler::CoreGuard::block_on::{{closure}}
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/basic_scheduler.rs:542:36
    63: tokio::runtime::basic_scheduler::CoreGuard::enter::{{closure}}
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/basic_scheduler.rs:613:57
    64: tokio::macros::scoped_tls::ScopedKey<T>::set
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/macros/scoped_tls.rs:61:9
    65: tokio::runtime::basic_scheduler::CoreGuard::enter
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/basic_scheduler.rs:613:27
    66: tokio::runtime::basic_scheduler::CoreGuard::block_on
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/basic_scheduler.rs:533:19
    67: tokio::runtime::basic_scheduler::BasicScheduler::block_on
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/basic_scheduler.rs:179:24
    68: tokio::runtime::Runtime::block_on
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-1.20.1/src/runtime/mod.rs:482:46
    69: actix_server::worker::ServerWorker::start::{{closure}}
    at /Users/xlange/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-server-2.1.1/src/worker.rs:402:29
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.