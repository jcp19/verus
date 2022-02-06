#[allow(unused_imports)]
use builtin::*;
#[allow(unused_imports)]
use crate::pervasive::*;
#[allow(unused_imports)]
use crate::pervasive::map::*;

/// set type for specifications
#[verifier(external_body)]
pub struct Set<#[verifier(maybe_negative)] A> {
    dummy: std::marker::PhantomData<A>,
}

#[spec]
#[verifier(external_body)]
pub fn set_new<A, F: Fn(A) -> bool>(f: F) -> Set<A> {
    unimplemented!()
}

impl<A> Set<A> {
    #[spec]
    #[verifier(external_body)]
    pub fn empty() -> Set<A> {
        unimplemented!()
    }

    #[spec]
    pub fn full() -> Set<A> {
        Set::empty().complement()
    }

    #[spec]
    #[verifier(external_body)]
    pub fn contains(self, a: A) -> bool {
        unimplemented!()
    }

    #[spec]
    pub fn ext_equal(self, s2: Set<A>) -> bool {
        forall(|a: A| self.contains(a) == s2.contains(a))
    }

    #[spec]
    pub fn subset_of(self, s2: Set<A>) -> bool {
        forall(|a: A| self.contains(a) >>= s2.contains(a))
    }

    #[spec]
    #[verifier(external_body)]
    pub fn insert(self, a: A) -> Set<A> {
        unimplemented!()
    }

    #[spec]
    #[verifier(external_body)]
    pub fn remove(self, a: A) -> Set<A> {
        unimplemented!()
    }

    #[spec]
    #[verifier(external_body)]
    pub fn union(self, s2: Set<A>) -> Set<A> {
        unimplemented!()
    }

    #[spec]
    #[verifier(external_body)]
    pub fn intersect(self, s2: Set<A>) -> Set<A> {
        unimplemented!()
    }

    #[spec]
    #[verifier(external_body)]
    pub fn difference(self, s2: Set<A>) -> Set<A> {
        unimplemented!()
    }

    #[spec]
    #[verifier(external_body)]
    pub fn complement(self) -> Set<A> {
        unimplemented!()
    }

    #[spec]
    pub fn filter<F: Fn(A) -> bool>(self, f: F) -> Set<A> {
        self.intersect(set_new(f))
    }

    #[spec]
    #[verifier(external_body)]
    pub fn finite(self) -> bool {
        unimplemented!()
    }

    #[spec]
    #[verifier(external_body)]
    pub fn len(self) -> nat {
        unimplemented!()
    }

    #[spec]
    pub fn choose(self) -> A {
        choose(|a: A| self.contains(a))
    }

    #[spec]
    #[verifier(external_body)]
    pub fn mk_map<V, F: Fn(A) -> V>(self, f: F) -> Map<A, V> {
        unimplemented!()
    }

}

// Trusted axioms

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_empty<A>(a: A) {
    ensures(!Set::empty().contains(a));
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_new<A, F: Fn(A) -> bool>(f: F, a: A) {
    ensures(set_new(f).contains(a) == f(a));
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_insert_same<A>(s: Set<A>, a: A) {
    ensures(#[trigger] s.insert(a).contains(a));
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_insert_different<A>(s: Set<A>, a1: A, a2: A) {
    requires(!equal(a1, a2));
    ensures(s.insert(a2).contains(a1) == s.contains(a1));
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_remove_same<A>(s: Set<A>, a: A) {
    ensures(!(#[trigger] s.remove(a).contains(a)));
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_remove_different<A>(s: Set<A>, a1: A, a2: A) {
    requires(!equal(a1, a2));
    ensures(s.remove(a2).contains(a1) == s.contains(a1));
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_union<A>(s1: Set<A>, s2: Set<A>, a: A) {
    ensures(s1.union(s2).contains(a) == (s1.contains(a) || s2.contains(a)));
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_intersect<A>(s1: Set<A>, s2: Set<A>, a: A) {
    ensures(s1.intersect(s2).contains(a) == (s1.contains(a) && s2.contains(a)));
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_difference<A>(s1: Set<A>, s2: Set<A>, a: A) {
    ensures(s1.difference(s2).contains(a) == (s1.contains(a) && !s2.contains(a)));
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_complement<A>(s: Set<A>, a: A) {
    ensures(s.complement().contains(a) == !s.contains(a));
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_ext_equal<A>(s1: Set<A>, s2: Set<A>) {
    ensures(s1.ext_equal(s2) == equal(s1, s2));
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_mk_map_domain<K, V, F: Fn(K) -> V>(s: Set<K>, f: F) {
    ensures(equal(#[trigger] s.mk_map(f).dom(), s));
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_mk_map_index<K, V, F: Fn(K) -> V>(s: Set<K>, f: F, key: K) {
    requires(s.contains(key));
    ensures(equal(s.mk_map(f).index(key), f(key)));
}

// Trusted axioms about finite

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_empty_finite<A>() {
    ensures(#[trigger] Set::<A>::empty().finite());
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_insert_finite<A>(s: Set<A>, a: A) {
    requires(s.finite());
    ensures(#[trigger] s.insert(a).finite());
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_remove_finite<A>(s: Set<A>, a: A) {
    requires(s.finite());
    ensures(#[trigger] s.remove(a).finite());
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_union_finite<A>(s1: Set<A>, s2: Set<A>) {
    requires([
        s1.finite(),
        s2.finite(),
    ]);
    ensures(#[trigger] s1.union(s2).finite());
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_intersect_finite<A>(s1: Set<A>, s2: Set<A>) {
    requires(s1.finite() || s2.finite());
    ensures(#[trigger] s1.intersect(s2).finite());
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_difference_finite<A>(s1: Set<A>, s2: Set<A>) {
    requires(s1.finite());
    ensures(#[trigger] s1.difference(s2).finite());
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_choose_finite<A>(s: Set<A>) {
    requires(!s.finite());
    ensures(#[trigger] s.contains(s.choose()));
}

// Trusted axioms about len

// Note: we could add more axioms about len, but they would be incomplete.
// The following, with axiom_set_ext_equal, are enough to build libraries about len.

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_empty_len<A>() {
    ensures([
        #[trigger] Set::<A>::empty().len() == 0,
    ]);
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_insert_len<A>(s: Set<A>, a: A) {
    requires([
        s.finite(),
    ]);
    ensures(#[trigger] s.insert(a).len() ==
        s.len() + (if s.contains(a) { 0 } else { 1 })
    );
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_remove_len<A>(s: Set<A>, a: A) {
    requires(s.finite());
    ensures(s.len() ==
        #[trigger] s.remove(a).len() + (if s.contains(a) { 1 } else { 0 })
    );
}

#[proof]
#[verifier(external_body)]
#[verifier(broadcast_forall)]
pub fn axiom_set_choose_len<A>(s: Set<A>) {
    requires([
        s.finite(),
        #[trigger] s.len() != 0,
    ]);
    ensures(#[trigger] s.contains(s.choose()));
}

// Macros

#[macro_export]
macro_rules! set_insert_rec {
    [$val:expr;] => {
        $val
    };
    [$val:expr;$elem:expr] => {
        set_insert_rec![$val.insert($elem);]
    };
    [$val:expr;$elem:expr,$($tail:tt)*] => {
        set_insert_rec![$val.insert($elem);$($tail)*]
    }
}

#[macro_export]
macro_rules! set {
    [$($tail:tt)*] => {
        set_insert_rec![$crate::pervasive::set::Set::empty();$($tail)*]
    }
}
