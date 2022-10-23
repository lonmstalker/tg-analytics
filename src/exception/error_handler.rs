use std::fmt::Debug;
use std::sync::Arc;
use futures::future::BoxFuture;
use teloxide::error_handlers::ErrorHandler;

pub struct BotErrorHandler {}

impl<T> ErrorHandler<T> for BotErrorHandler
    where T: Debug, {
    fn handle_error(self: Arc<Self>, error: T) -> BoxFuture<'static, ()> {
        let text = format!("bot error: {:?}", error);
        log::error!("{}", text);
        Box::pin(
            async move { drop(text); }
        )
    }
}