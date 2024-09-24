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

pub struct MutSetRef<'a, T: Ord> {
    set: &'a mut BTreeSet<T>,
    item: Option<T>,
}
impl<T: Ord> Drop for MutSetRef<'_, T> {
    fn drop(&mut self) {
        self.set.insert(self.item.take().unwrap());
    }
}
impl<T: Ord> Deref for MutSetRef<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.item.as_ref().unwrap()
    }
}
impl<T: Ord> DerefMut for MutSetRef<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.item.as_mut().unwrap()
    }
}

pub trait MutSet<T> {
    fn get_mut<'a, K>(&'a mut self, k: &K) -> Option<MutSetRef<'a, T>> where T: Ord + Borrow<K>, K: Ord + ?Sized;
}
impl<T: Ord> MutSet<T> for BTreeSet<T> {
    fn get_mut<'a, K>(&'a mut self, k: &K) -> Option<MutSetRef<'a, T>> where T: Ord + Borrow<K>, K: Ord + ?Sized {
        self.take(k).map(|v| MutSetRef { set: self, item: Some(v) })
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
