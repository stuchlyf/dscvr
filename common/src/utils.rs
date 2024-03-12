use std::thread::{Scope, ScopedJoinHandle};
use std::{io, thread};

pub fn spawn_scoped_thread_with_name<'env, 'scope, F, T>(
    s: &'scope Scope<'scope, 'env>,
    name: String,
    function: F,
) -> io::Result<ScopedJoinHandle<'scope, T>>
where
    F: FnOnce() -> T + Send + 'scope,
    T: Send + 'scope,
{
    thread::Builder::new().name(name).spawn_scoped(s, function)
}
