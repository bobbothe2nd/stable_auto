use crate::{AllImpl, Auto, Combined};

unsafe impl<A> Auto for (A,) {
    type Fields = AllImpl<A>;
}
unsafe impl<A,B> Auto for (A,B) {
    type Fields = Combined<A, B>;
}
unsafe impl<A,B,C> Auto for (A,B,C) {
    type Fields = Combined<Combined<A, B>, C>;
}
unsafe impl<A,B,C,D> Auto for (A,B,C,D) {
    type Fields = Combined<Combined<A,B>,Combined<C,D>>;
}
unsafe impl<A,B,C,D,E> Auto for (A,B,C,D,E) {
    type Fields = Combined<Combined<Combined<A,E>,B>,Combined<C,D>>;
}
unsafe impl<A,B,C,D,E,F> Auto for (A,B,C,D,E,F) {
    type Fields = Combined<Combined<Combined<A,E>,Combined<B,F>>,Combined<C,D>>;
}
unsafe impl<A,B,C,D,E,F,G> Auto for (A,B,C,D,E,F,G) {
    type Fields = Combined<Combined<Combined<A,E>,Combined<B,F>>,Combined<Combined<C,G>,D>>;
}
unsafe impl<A,B,C,D,E,F,G,H> Auto for (A,B,C,D,E,F,G,H) {
    type Fields = Combined<Combined<Combined<A,E>,Combined<B,F>>,Combined<Combined<C,G>,Combined<D,H>>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I> Auto for (A,B,C,D,E,F,G,H,I) {
    type Fields = Combined<Combined<Combined<Combined<A,I>,E>,Combined<B,F>>,Combined<Combined<C,G>,Combined<D,H>>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J> Auto for (A,B,C,D,E,F,G,H,I,J) {
    type Fields = Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<B,F>>,Combined<Combined<C,G>,Combined<D,H>>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J,K> Auto for (A,B,C,D,E,F,G,H,I,J,K) {
    type Fields = Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<Combined<B,K>,F>>,Combined<Combined<C,G>,Combined<D,H>>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J,K,L> Auto for (A,B,C,D,E,F,G,H,I,J,K,L) {
    type Fields = Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<Combined<B,K>,Combined<F,L>>>,Combined<Combined<C,G>,Combined<D,H>>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J,K,L,M> Auto for (A,B,C,D,E,F,G,H,I,J,K,L,M) {
    type Fields = Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<Combined<B,K>,Combined<F,L>>>,Combined<Combined<C,G>,Combined<Combined<D,M>,H>>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N> Auto for (A,B,C,D,E,F,G,H,I,J,K,L,M,N) {
    type Fields = Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<Combined<B,K>,Combined<F,L>>>,Combined<Combined<Combined<C,G>,Combined<D,M>>,Combined<H,N>>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O> Auto for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O) {
    type Fields = Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<Combined<B,K>,Combined<F,L>>>,Combined<Combined<Combined<C,G>,Combined<D,M>>,Combined<Combined<H,N>,O>>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P> Auto for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P) {
    type Fields = Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<Combined<B,K>,Combined<F,L>>>,Combined<Combined<Combined<C,G>,Combined<D,M>>,Combined<Combined<H,N>,Combined<O,P>>>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q> Auto for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q) {
    type Fields = Combined<Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<Combined<B,K>,Combined<F,L>>>,Combined<Combined<Combined<C,G>,Combined<D,M>>,Combined<Combined<H,N>,Combined<O,P>>>>,Q>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R> Auto for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R) {
    type Fields = Combined<Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<Combined<B,K>,Combined<F,L>>>,Combined<Combined<Combined<C,G>,Combined<D,M>>,Combined<Combined<H,N>,Combined<O,P>>>>,Combined<Q,R>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S> Auto for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S) {
    type Fields = Combined<Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<Combined<B,K>,Combined<F,L>>>,Combined<Combined<Combined<C,G>,Combined<D,M>>,Combined<Combined<H,N>,Combined<O,P>>>>,Combined<Combined<Q,R>,S>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T> Auto for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T) {
    type Fields = Combined<Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<Combined<B,K>,Combined<F,L>>>,Combined<Combined<Combined<C,G>,Combined<D,M>>,Combined<Combined<H,N>,Combined<O,P>>>>,Combined<Combined<Q,R>,Combined<S,T>>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U> Auto for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U) {
    type Fields = Combined<Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<Combined<B,K>,Combined<F,L>>>,Combined<Combined<Combined<C,G>,Combined<D,M>>,Combined<Combined<H,N>,Combined<O,P>>>>,Combined<Combined<Combined<Q,R>,Combined<S,T>>,U>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V> Auto for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V) {
    type Fields = Combined<Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<Combined<B,K>,Combined<F,L>>>,Combined<Combined<Combined<C,G>,Combined<D,M>>,Combined<Combined<H,N>,Combined<O,P>>>>,Combined<Combined<Combined<Q,R>,Combined<S,T>>,Combined<U,V>>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W> Auto for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W) {
    type Fields = Combined<Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<Combined<B,K>,Combined<F,L>>>,Combined<Combined<Combined<C,G>,Combined<D,M>>,Combined<Combined<H,N>,Combined<O,P>>>>,Combined<Combined<Combined<Q,R>,Combined<S,T>>,Combined<Combined<U,V>,W>>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X> Auto for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X) {
    type Fields = Combined<Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<Combined<B,K>,Combined<F,L>>>,Combined<Combined<Combined<C,G>,Combined<D,M>>,Combined<Combined<H,N>,Combined<O,P>>>>,Combined<Combined<Combined<Q,R>,Combined<S,T>>,Combined<Combined<U,V>,Combined<W,X>>>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y> Auto for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y) {
    type Fields = Combined<Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<Combined<B,K>,Combined<F,L>>>,Combined<Combined<Combined<C,G>,Combined<D,M>>,Combined<Combined<H,N>,Combined<O,P>>>>,Combined<Combined<Combined<Q,R>,Combined<S,T>>,Combined<Combined<Combined<U,V>,Combined<W,X>>,Y>>>;
}
unsafe impl<A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z> Auto for (A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z) {
    type Fields = Combined<Combined<Combined<Combined<Combined<A,I>,Combined<E,J>>,Combined<Combined<B,K>,Combined<F,L>>>,Combined<Combined<Combined<C,G>,Combined<D,M>>,Combined<Combined<H,N>,Combined<O,P>>>>,Combined<Combined<Combined<Q,R>,Combined<S,T>>,Combined<Combined<Combined<U,V>,Combined<W,X>>,Combined<Y,Z>>>>;
}
