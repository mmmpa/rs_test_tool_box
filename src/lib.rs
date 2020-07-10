#![allow(warnings)]
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use rand::Rng;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

pub fn random_string(l: usize) -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(l)
        .collect()
}

#[cfg(feature = "initialize_pretty_env_logger")]
pub mod pretty {
    use crate::once::initialize_once_async;

    pub async fn initialize_pretty_env_logger_once() {
        initialize_once_async("pretty_env_logger", || async {
            pretty_env_logger::init();
            info!("initialized logger!")
        })
        .await;
    }
}

#[cfg(feature = "initialize_once")]
pub mod once {
    use std::collections::HashMap;
    use tokio::macros::support::Future;
    use tokio::sync::RwLock;

    lazy_static! {
        pub static ref initialized: RwLock<HashMap<String, bool>> = { Default::default() };
    }

    pub async fn initialize_once<F, R>(name: impl Into<String>, f: F)
    where
        F: FnOnce() -> R,
    {
        let name = name.into();
        let mut lock = initialized.write().await;
        if let Some(true) = lock.get(&name) {
            return;
        } else {
            info!("{} initialized.", name);
            lock.insert(name, true);
        }

        f();
    }

    pub async fn initialize_once_async<F, Fut, R>(name: impl Into<String>, f: F)
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = R>,
    {
        let name = name.into();
        let mut lock = initialized.write().await;
        if let Some(true) = lock.get(&name) {
            return;
        } else {
            info!("\"{}\" initialized.", name);
            lock.insert(name, true);
        }

        f().await;
    }

    #[cfg(test)]
    mod tests {
        use crate::once::initialize_once_async;
        use futures::future::join_all;

        static mut COUNT: usize = 0;

        #[tokio::test]
        async fn test_async() {
            let mut ts = vec![];
            for _ in 0..3 {
                let t = tokio::spawn(async {
                    initialize_once_async("count", || async { unsafe { COUNT += 1 } }).await;
                });
                ts.push(t);
            }
            let end = join_all(ts).await;
            assert_eq!(3, end.len());
            assert_eq!(1, unsafe { COUNT });
        }
    }
}
