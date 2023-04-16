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
trait Functor<'a, A> {
    type Orig;
    // type Inner;
    type Target<Inner>;
    fn functor_map<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B + 'a;
}
use higher::Functor as HighFunctor;

// impl<A: Default> HighFunctor<'_, A> for Defaulted<A> {
// type Target<T: Default> = Defaulted<T>;
impl<A: Default> Functor<'_, A> for Defaulted<A> {
    type Orig = A;
    // type Inner = <Defaulted<B> as Functor<'_, B>>::Orig;
    type Target<Inner> = Defaulted<<Defaulted<Inner> as Functor<'_, Inner>>::Orig>;
fn functor_map<B, F>(self, f: F) -> Self::Target<B>
where
    F: Fn(A) -> B, B: Default{
match self {
    Defaulted::Arbitrary(x) => Defaulted::Arbitrary(f(x)),
    Defaulted::Default => Defaulted::Arbitrary(B::default()),
}
}
//
// fn functor_map<B, F>(self, f: F) -> Self::Target<B>
// fn fmap<B, F>(self, f: F) -> Defaulted<<Defaulted<A> as higher::Functor<'_, A>>::Target<B>>
// fn fmap<B, F>(self, f: F) -> Self::Target<T>
// where
//     F: Fn(A) -> B,
//     B: std::default::Default, // B: Default,
// {
//     self.map(f)
// match self {
//     Defaulted::Arbitrary(x) => Defaulted::Arbitrary(f(x)),
//     Defaulted::Default => Defaulted::Arbitrary(B::default()),
// }
//     }
// }

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

// trait Functor {
//     type Inner;
//     // type Output<Wrapped>: Functor;
//     // type Wrapped;
//     type Output<Wrapped>: Functor;
//
//     // fn functor_map<F, Wrapped>(self, f: F) -> Self::Output<Wrapped>
//     fn functor_map<F, Wrapped>(self, f: F) -> Self::Output<Wrapped>
//     where
//         F: FnMut(Self::Inner) -> Wrapped;
//     // or:
//     // fn functor_map<F: FnMut(Self::Inner) -> Wrapped, Wrapped>(self, f: F) -> Self::Output<Wrapped>;
// }

// impl<A: Default> Functor for Defaulted<A> {
//     type Inner = A;
//     // type Wrapped = B;
//     type Output<Wrapped> = Defaulted<impl Default>;
//
//     fn functor_map<F, Wrapped>(self, mut f: F) -> Self::Output<Wrapped>
//     where
//         F: FnMut(Self::Inner) -> Wrapped,
//         Wrapped: Default,
//     {
//         match self {
//             Defaulted::Arbitrary(x) => Defaulted::Arbitrary(f(x)),
//             Defaulted::Default => Defaulted::Arbitrary(Wrapped::default()),
//         }
//     }
// }

// impl<A: Default> Functor for Defaulted<A> {
//     type Inner = A;
//     // type Wrapped = B;
//     type Output<Wrapped> = Defaulted<impl Default>;
//
//     fn functor_map<F, X: Default>(self, mut f: F) -> Self::Output<X>
//     where
//         F: FnMut(Self::Inner) -> X,
//     {
//         match self {
//             Defaulted::Arbitrary(x) => Defaulted::Arbitrary(f(x)),
//             Defaulted::Default => Defaulted::Arbitrary(X::default()),
//         }
//     }
// }
//
// impl<X: Default, Y: Default> Functor for Defaulted<X> {
//     type Inner = X;
//     type Wrapped = Y;
//     type Output = Defaulted<Y>;
//
//     fn functor_map<F: FnMut(X) -> Wrapped, Wrapped: Default>(self, f: F) -> Defaulted<Wrapped> {
//         match self {
//             Defaulted::Arbitrary(x) => Defaulted::Arbitrary(f(x)),
//             Defaulted::Default => Defaulted::Arbitrary(Wrapped::default()),
//         }
//     }
// }
// impl<T, Wrapped> Functor for Defaulted<T>
// impl<T, S> Functor for Defaulted<T>
// where
//     T: Default,
//     S: Default,
// {
//     type Inner = T;
//     type Output<Wrapped> = Defaulted<S>;
//
//     fn functor_map<F: FnMut(<T: Default>) -> S:Default, S>(self, f: F) -> Defaulted<S> {
//         match self {
//             Defaulted::Arbitrary(x) => Defaulted::Arbitrary(f(x)),
//             Defaulted::Default => Defaulted::Arbitrary(S::default()),
//         }
//     }
// }

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
}
