use alloc::collections::BTreeSet;
use core::borrow::Borrow;
use core::ops::{Deref, DerefMut};

pub struct MappedRef<T: Deref + DerefMut, V> {
    t: T,
    f: fn(&T::Target) -> &V,
    g: fn(&mut T::Target) -> &mut V,
}
impl<T: Deref + DerefMut, V> Deref for MappedRef<T, V> {
    type Target = V;
    fn deref(&self) -> &Self::Target {
        (self.f)(self.t.deref())
    }
}
impl<T: Deref + DerefMut, V> DerefMut for MappedRef<T, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        (self.g)(self.t.deref_mut())
    }
}

pub trait MapRef: Deref + DerefMut + Sized {
    fn map_ref<V>(self, ref_fn: fn(&Self::Target) -> &V, mut_fn: fn(&mut Self::Target) -> &mut V) -> MappedRef<Self, V>;
}
impl<T: Deref + DerefMut> MapRef for T {
    fn map_ref<V>(self, ref_fn: fn(&Self::Target) -> &V, mut_fn: fn(&mut Self::Target) -> &mut V) -> MappedRef<Self, V> {
        MappedRef { t: self, f: ref_fn, g: mut_fn }
    }
}

pub enum MutRefInner<'a, T: Ord> {
    Set { src: &'a mut BTreeSet<T>, val: Option<T> },
    Normal { val: &'a mut T },
}
pub struct MutRef<'a, T: Ord>(MutRefInner<'a, T>);
impl<T: Ord> Drop for MutRef<'_, T> {
    fn drop(&mut self) {
        match &mut self.0 {
            MutRefInner::Set { src, val } => {
                src.insert(val.take().unwrap());
            }
            MutRefInner::Normal { val: _ } => (),
        }
    }
}
impl<T: Ord> Deref for MutRef<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match &self.0 {
            MutRefInner::Set { src: _, val } => val.as_ref().unwrap(),
            MutRefInner::Normal { val } => val,
        }
    }
}
impl<T: Ord> DerefMut for MutRef<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut self.0 {
            MutRefInner::Set { src: _, val } => val.as_mut().unwrap(),
            MutRefInner::Normal { val } => val,
        }
    }
}

pub struct MutSetIter<'a, T: Ord> {
    src: &'a mut BTreeSet<T>,
    rev_vals: Vec<T>,
}
impl<'a, T: Ord> Iterator for MutSetIter<'a, T> {
    type Item = MutRef<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.rev_vals.pop() {
            let res = self.src.get_mut(&v);
            if res.is_some() {
                return res;
            }
        }
        None
    }
}

pub trait MutSet<T> {
    fn get_mut<'a, K>(&'a mut self, k: &K) -> Option<MutRef<'a, T>> where T: Ord + Borrow<K>, K: Ord + ?Sized;
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = MutRef<'a, T>> where T: Ord + Clone + 'a;
}
impl<T: Ord> MutSet<T> for BTreeSet<T> {
    fn get_mut<'a, K>(&'a mut self, k: &K) -> Option<MutRef<'a, T>> where T: Ord + Borrow<K>, K: Ord + ?Sized {
        self.take(k).map(|v| MutRef(MutRefInner::Set { src: self, val: Some(v) }))
    }
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = MutRef<'a, T>> where T: Ord + Clone + 'a {
        MutSetIter { rev_vals: self.iter().rev().cloned().collect(), src: self }
    }
}

#[test]
fn test_mut_set() {
    let mut v = BTreeSet::<usize>::new();
    v.insert(5);
    v.insert(7);
    assert_eq!(v.iter().copied().collect::<Vec<_>>(), &[5, 7]);
    *v.get_mut(&5).unwrap() = 3;
    assert_eq!(v.iter().copied().collect::<Vec<_>>(), &[3, 7]);
    *v.get_mut(&7).unwrap() = 11;
    assert_eq!(v.iter().copied().collect::<Vec<_>>(), &[3, 11]);
    assert!(v.get_mut(&5).is_none());
}
