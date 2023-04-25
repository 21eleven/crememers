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

trait Applicative<A, B, C>: Pointed<A, C> + Pointed<A, B> {
    fn lift_a2<F>(
        self,
        f: F,
        b: <Self as Functor<A, B>>::Target<B>,
    ) -> <Self as Functor<A, C>>::Target<C>
    where
        F: Fn(A, B) -> C;
}

impl<A: Default, B: Default, C: Default> Applicative<A, B, C> for Defaulted<A> {
    fn lift_a2<F>(
        self,
        f: F,
        b: <Self as Functor<A, B>>::Target<B>,
    ) -> <Self as Functor<A, C>>::Target<C>
    where
        F: Fn(A, B) -> C,
    {
        match (&self, &b) {
            (Defaulted::Default, Defaulted::Default) => Defaulted::Default,
            _ => Defaulted::Arbitrary(f(self.incorporate(), b.incorporate())),
        }
    }
}

trait Semigroup {
    fn append(self, rhs: Self) -> Self;
}

impl Semigroup for () {
    fn append(self, unit: ()) -> () {}
}

impl<T> Semigroup for Vec<T> {
    fn append(mut self, mut rhs: Self) -> Self {
        Vec::append(&mut self, &mut rhs);
        self
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Validation<A, M> {
    Valid(A),
    Mangled(M),
}

impl<A, B, M> Functor<A, B> for Validation<A, M> {
    type Target<X> = Validation<B, M>;
    fn func_map<F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B,
    {
        match self {
            Validation::Valid(a) => Validation::Valid(f(a)),
            Validation::Mangled(m) => Validation::Mangled(m),
        }
    }
}

impl<Void, A, M> Pointed<Void, A> for Validation<Void, M>
where
    Validation<A, M>: Functor<Void, A>,
{
    fn point(pt: A) -> Self::Target<A> {
        Validation::Valid(pt)
    }
}

impl<A, M: Semigroup, B, C> Applicative<A, B, C> for Validation<A, M>
where
    Validation<B, M>: Functor<A, B>,
    Validation<C, M>: Functor<A, C>,
{
    fn lift_a2<F>(
        self,
        f: F,
        b: <Self as Functor<A, B>>::Target<B>,
    ) -> <Self as Functor<A, C>>::Target<C>
    where
        F: Fn(A, B) -> C,
    {
        match (self, b) {
            (Validation::Valid(a), Validation::Valid(b)) => Validation::Valid(f(a, b)),
            (Validation::Mangled(m), Validation::Valid(_)) => Validation::Mangled(m),
            (Validation::Valid(_), Validation::Mangled(m)) => Validation::Mangled(m),
            (Validation::Mangled(m1), Validation::Mangled(m2)) => {
                Validation::Mangled(m1.append(m2))
            }
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
        assert_eq!(a.clone().lift_a2(combine, b), cab);
        assert_eq!(a.clone().lift_a2(combine, b.reset()), ca);
        assert_eq!(Defaulted::Default.lift_a2(combine, b), cb);
        assert_eq!(Defaulted::Default.lift_a2(combine, b.reset()), cd);
    }

    #[test]
    fn semigroup() {
        let a = Validation::<usize, Vec<String>>::Valid(3);
        let b = Validation::<usize, Vec<String>>::Valid(7);
        let x = Validation::<usize, Vec<String>>::Mangled(vec!["oh".to_string()]);
        let y = Validation::<usize, Vec<String>>::Mangled(vec!["no".to_string()]);

        let add = |i: usize, j: usize| i + j;

        assert_eq!(a.clone().lift_a2(add, b), Validation::Valid(10));
        assert_eq!(a.lift_a2(add, y.clone()), y);
        assert_eq!(
            x.lift_a2(add, y),
            Validation::Mangled(vec!["oh".to_string(), "no".to_string()])
        );
    }
}
