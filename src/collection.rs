// use std::collections::HashMap;
// use crate::resource_definition::SerDe;
//
// pub trait Collection<T> {
//     fn get_key(&self, key: &str) -> Option<&T>;
//     fn insert() {}
//     fn extend() {}
// }
//
// pub struct SimpleCollection<T> where T: SerDe + Sized {
//     v: Vec<T>
// }
//
// impl<T: SerDe + Sized> Collection<T> for SimpleCollection<T> {
//     fn get_key(&self, key: &str) -> Option<&T> {
//         for rd in self.v.iter() {
//             if rd.id() == key {
//                 return Some(rd);
//             }
//         }
//         None
//     }
// }

// pub struct ComposedCollection<T> where T: Collection<T> {
//
// }
//
// impl<T> Collection<T> for ComposedCollection<T> where T: Collection<T> {
//     fn get_key(&self, key: &str) -> Option<&S> {
//         None
//     }
// }
