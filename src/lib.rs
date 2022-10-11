mod error;
mod future;
mod retry_strategy;

pub use error::{RetryError, TimeoutError};
pub use future::{FutureFactory, FutureRetry};
pub use retry_strategy::{
    ExponentialRetryStrategy, InfiniteRetryStrategy, LinearRetryStrategy, RetryStrategy,
};

#[derive(Debug)]
pub enum RetryPolicy<E> {
    Repeat,
    Fail(E),
    Any(anyhow::Error),
}

impl<E, T: Into<anyhow::Error>> From<T> for RetryPolicy<E> {
    fn from(t: T) -> Self {
        Self::Any(t.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{
        future::{err, ok},
        TryFutureExt,
    };

    #[tokio::test]
    async fn test_ok() {
        let f = FutureRetry::new(
            || ok::<_, u8>(255).map_err(|_| RetryPolicy::Repeat::<String>),
            LinearRetryStrategy::default(),
        );
        assert_eq!(255, f.await.unwrap());
    }

    #[tokio::test]
    async fn test_error() {
        let f = FutureRetry::new(
            || err::<u8, _>(RetryPolicy::Fail("fail")),
            LinearRetryStrategy::default().attempts(1),
        );
        assert_eq!(f.await.unwrap_err().to_string(), "Fail: fail");
    }
}
