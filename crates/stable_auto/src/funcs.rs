impl_auto_leaf!(
    <Ret> fn() -> Ret
    <Ret, A> fn(A) -> Ret
    <Ret, A, B> fn(A, B) -> Ret
    <Ret, A, B, C> fn(A, B, C) -> Ret
    <Ret, A, B, C, D> fn(A, B, C, D) -> Ret
    <Ret, A, B, C, D, E> fn(A, B, C, D, E) -> Ret
    <Ret, A, B, C, D, E, F> fn(A, B, C, D, E, F) -> Ret
    <Ret, A, B, C, D, E, F, G> fn(A, B, C, D, E, F, G) -> Ret
    <Ret, A, B, C, D, E, F, G, H> fn(A, B, C, D, E, F, G, H) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I> fn(A, B, C, D, E, F, G, H, I) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J> fn(A, B, C, D, E, F, G, H, I, J) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K> fn(A, B, C, D, E, F, G, H, I, J, K) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L> fn(A, B, C, D, E, F, G, H, I, J, K, L) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M> fn(A, B, C, D, E, F, G, H, I, J, K, L, M) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N> fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q> fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R> fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S> fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T> fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U> fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V> fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W> fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X> fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y> fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z> fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z) -> Ret
    <Ret> extern "C" fn() -> Ret
    <Ret, A> extern "C" fn(A) -> Ret
    <Ret, A, B> extern "C" fn(A, B) -> Ret
    <Ret, A, B, C> extern "C" fn(A, B, C) -> Ret
    <Ret, A, B, C, D> extern "C" fn(A, B, C, D) -> Ret
    <Ret, A, B, C, D, E> extern "C" fn(A, B, C, D, E) -> Ret
    <Ret, A, B, C, D, E, F> extern "C" fn(A, B, C, D, E, F) -> Ret
    <Ret, A, B, C, D, E, F, G> extern "C" fn(A, B, C, D, E, F, G) -> Ret
    <Ret, A, B, C, D, E, F, G, H> extern "C" fn(A, B, C, D, E, F, G, H) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I> extern "C" fn(A, B, C, D, E, F, G, H, I) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J> extern "C" fn(A, B, C, D, E, F, G, H, I, J) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K> extern "C" fn(A, B, C, D, E, F, G, H, I, J, K) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L> extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M> extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L, M) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N> extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q> extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R> extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S> extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T> extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U> extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V> extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W> extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X> extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y> extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z> extern "C" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z) -> Ret
    <Ret> extern "system" fn() -> Ret
    <Ret, A> extern "system" fn(A) -> Ret
    <Ret, A, B> extern "system" fn(A, B) -> Ret
    <Ret, A, B, C> extern "system" fn(A, B, C) -> Ret
    <Ret, A, B, C, D> extern "system" fn(A, B, C, D) -> Ret
    <Ret, A, B, C, D, E> extern "system" fn(A, B, C, D, E) -> Ret
    <Ret, A, B, C, D, E, F> extern "system" fn(A, B, C, D, E, F) -> Ret
    <Ret, A, B, C, D, E, F, G> extern "system" fn(A, B, C, D, E, F, G) -> Ret
    <Ret, A, B, C, D, E, F, G, H> extern "system" fn(A, B, C, D, E, F, G, H) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I> extern "system" fn(A, B, C, D, E, F, G, H, I) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J> extern "system" fn(A, B, C, D, E, F, G, H, I, J) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K> extern "system" fn(A, B, C, D, E, F, G, H, I, J, K) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L> extern "system" fn(A, B, C, D, E, F, G, H, I, J, K, L) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M> extern "system" fn(A, B, C, D, E, F, G, H, I, J, K, L, M) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N> extern "system" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> extern "system" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> extern "system" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q> extern "system" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R> extern "system" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S> extern "system" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T> extern "system" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U> extern "system" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V> extern "system" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W> extern "system" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X> extern "system" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y> extern "system" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z> extern "system" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z) -> Ret
    <Ret> extern "win64" fn() -> Ret
    <Ret, A> extern "win64" fn(A) -> Ret
    <Ret, A, B> extern "win64" fn(A, B) -> Ret
    <Ret, A, B, C> extern "win64" fn(A, B, C) -> Ret
    <Ret, A, B, C, D> extern "win64" fn(A, B, C, D) -> Ret
    <Ret, A, B, C, D, E> extern "win64" fn(A, B, C, D, E) -> Ret
    <Ret, A, B, C, D, E, F> extern "win64" fn(A, B, C, D, E, F) -> Ret
    <Ret, A, B, C, D, E, F, G> extern "win64" fn(A, B, C, D, E, F, G) -> Ret
    <Ret, A, B, C, D, E, F, G, H> extern "win64" fn(A, B, C, D, E, F, G, H) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I> extern "win64" fn(A, B, C, D, E, F, G, H, I) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J> extern "win64" fn(A, B, C, D, E, F, G, H, I, J) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K> extern "win64" fn(A, B, C, D, E, F, G, H, I, J, K) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L> extern "win64" fn(A, B, C, D, E, F, G, H, I, J, K, L) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M> extern "win64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N> extern "win64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> extern "win64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> extern "win64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q> extern "win64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R> extern "win64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S> extern "win64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T> extern "win64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U> extern "win64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V> extern "win64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W> extern "win64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X> extern "win64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y> extern "win64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z> extern "win64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z) -> Ret
    <Ret> extern "sysv64" fn() -> Ret
    <Ret, A> extern "sysv64" fn(A) -> Ret
    <Ret, A, B> extern "sysv64" fn(A, B) -> Ret
    <Ret, A, B, C> extern "sysv64" fn(A, B, C) -> Ret
    <Ret, A, B, C, D> extern "sysv64" fn(A, B, C, D) -> Ret
    <Ret, A, B, C, D, E> extern "sysv64" fn(A, B, C, D, E) -> Ret
    <Ret, A, B, C, D, E, F> extern "sysv64" fn(A, B, C, D, E, F) -> Ret
    <Ret, A, B, C, D, E, F, G> extern "sysv64" fn(A, B, C, D, E, F, G) -> Ret
    <Ret, A, B, C, D, E, F, G, H> extern "sysv64" fn(A, B, C, D, E, F, G, H) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I> extern "sysv64" fn(A, B, C, D, E, F, G, H, I) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J, K) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J, K, L) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y) -> Ret
    <Ret, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z> extern "sysv64" fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z) -> Ret
);
