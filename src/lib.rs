#![allow(unused_variables)]
#![feature(associated_type_bounds)]
#![feature(associated_type_defaults)]
#![feature(type_alias_impl_trait)]
#![allow(dead_code)]
// #![allow(unused_variables)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
    fn incorporate(self) -> A {
        match self {
            Defaulted::Arbitrary(x) => x,
            Defaulted::Default => A::default(),
        }
    }
    fn reset(self) -> Defaulted<A> {
        Self::Default
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
    // type Inner;
    type Target<T>;
    fn func_map<F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B;
}

impl<A: Default, B: Default> Functor<A, B> for Defaulted<A> {
    // type Inner = A;
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

trait Applicative<A, B, C>: Pointed<A, C> {
    type FoldingIn<T>;
    fn lift_a2<F>(
        self,
        f: F,
        // b: <(dyn Pointed<A, B, Target<B> = Self::Target<C>> + 'static) as Functor<A, B>>::Target<B>,
        b: Self::FoldingIn<B>,
    ) -> Self::Target<C>
    where
        // Here A could alternatively be Functor::Inner
        F: Fn(A, B) -> C;
}

impl<A: Default, B: Default, C: Default> Applicative<A, B, C> for Defaulted<A>
where
    Defaulted<A>: Functor<A, B>,
{
    type FoldingIn<T> = Defaulted<B>;
    // type FoldingIn<T> = <(dyn Functor<A, B, Target<B> = Defaulted<B>>) as Functor<A, B>>::Target<B>;
    fn lift_a2<F>(
        self,
        f: F,
        // b: <(dyn Pointed<A, B, Target<B> = Defaulted<B>> + 'static) as Functor<A, B>>::Target<B>,
        // b: Defaulted<B>,
        b: Self::FoldingIn<B>,
    ) -> Self::Target<C>
    where
        // Here A could alternatively be Functor::Inner
        F: Fn(A, B) -> C,
    {
        match (&self, &b) {
            (Defaulted::Default, Defaulted::Default) => Defaulted::Default,
            _ => Defaulted::Arbitrary(f(self.incorporate(), b.incorporate())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, HashSet};

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
    fn point_out() {
        assert_eq!(
            Defaulted::<Vec<usize>>::point(vec![0usize, 1usize, 0usize]),
            Defaulted::Arbitrary(vec![0, 1, 0])
        );
        assert_ne!(
            Defaulted::<String>::point("yo".to_string()),
            Defaulted::Default
        );
    }
    #[test]
    fn applicative() {
        let a = Defaulted::<HashSet<usize>>::Arbitrary([21, 11].into());
        let b = Defaulted::<usize>::Arbitrary(44);
        let cab = Defaulted::<BTreeSet<usize>>::Arbitrary([44, 21, 11].into());
        let ca = Defaulted::<BTreeSet<usize>>::Arbitrary([0, 21, 11].into());
        let cb = Defaulted::<BTreeSet<usize>>::Arbitrary([44].into());
        let cd = Defaulted::<BTreeSet<usize>>::Default;
        fn combine<X: Copy + std::cmp::Ord>(inner_a: HashSet<X>, inner_b: X) -> BTreeSet<X> {
            inner_a
                .iter()
                .chain([inner_b].iter())
                .map(|n| n.to_owned())
                .collect()
        }
        // combine(a.incorporate(), b.incorporate());
        assert_eq!(a.clone().lift_a2(combine, b), cab);
        assert_eq!(a.clone().lift_a2(combine, b.reset()), ca);
        assert_eq!(Defaulted::Default.lift_a2(combine, b), cb);
        assert_eq!(Defaulted::Default.lift_a2(combine, b.reset()), cd);
    }
}
