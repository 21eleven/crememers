#![allow(unused_variables)]
#![feature(associated_type_bounds)]
#![feature(type_alias_impl_trait)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, PartialEq)]
enum Defaulted<A>
where
    A: Default,
{
    Arbitrary(A),
    Default,
}

impl<A: Default> Defaulted<A> {
    fn map<F: FnOnce(A) -> B, B: Default>(self, f: F) -> Defaulted<B> {
        match self {
            Defaulted::Arbitrary(x) => Defaulted::Arbitrary(f(x)),
            Defaulted::Default => Defaulted::Arbitrary(B::default()),
        }
    }
}

enum Probabilistic<N> {
    Varying(N),
    Constant,
}

// #[derive(Debug, PartialEq)]
enum Hides {
    Hidden,
    Present,
}

// runtime instantiation
// use std::marker::PhantomData;
enum Identity<S> {
    ElfSelf(S),
}

// Captures the idea of transitioning the inner
// type within some wrapper type from Wrapper<A>
// to Wrapper<B> using a function from A to B
trait Functor<A, B> {
    type Target<T>;
    fn func_map<F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B;
}

impl<A: Default, B: Default> Functor<A, B> for Defaulted<A> {
    type Target<T> = Defaulted<B>;
    fn func_map<F>(self, f: F) -> Defaulted<B>
    where
        F: Fn(A) -> B,
    {
        match self {
            Defaulted::Arbitrary(x) => Defaulted::Arbitrary(f(x)),
            Defaulted::Default => Defaulted::Arbitrary(B::default()),
        }
    }
}

// Pointed births a Targeted type into
// a functor land. The prior inner type
// is Void in a category theory sense.
// (or is it Unit?)
trait Pointed<Void, B>: Functor<Void, B> {
    fn point(pt: B) -> Self::Target<B>;
}

impl<Void: Default, B: Default> Pointed<Void, B> for Defaulted<Void> {
    fn point(pt: B) -> Self::Target<B> {
        Defaulted::Arbitrary(pt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn defaulted() {
        assert_eq!(
            Defaulted::Arbitrary("2111").func_map(|x| x.parse::<usize>().unwrap_or(100)),
            Defaulted::Arbitrary(2111)
        );
        assert_eq!(
            Defaulted::<Option<usize>>::Default.func_map(|x| x.unwrap()),
            Defaulted::Arbitrary(0)
        );
    }
    #[test]
    fn pointOut() {
        assert_eq!(
            Defaulted::<Vec<usize>>::point(vec![0usize, 1usize, 0usize]),
            Defaulted::Arbitrary(vec![0, 1, 0])
        );
        assert_ne!(
            Defaulted::<String>::point("yo".to_string()),
            Defaulted::Default
        );
    }
}
