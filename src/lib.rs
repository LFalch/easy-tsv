#![warn(clippy::all, missing_docs)]
//! Crate for doing tsv stuffs

use std::str::FromStr;

pub trait TsvValue: Sized {
    fn serialize(&self) -> String;
    fn deserialize(s: &str) -> Option<Self>;
}

impl<T: Sized + ToString + FromStr> TsvValue for T {
    fn serialize(&self) -> String {
        self.to_string()
    }
    fn deserialize(s: &str) -> Option<Self> {
        Self::from_str(s).ok()
    }
}

pub trait TsvSerialize: Sized {
    fn keys() -> &'static [&'static str];
    fn to_line(&self) -> String;
    fn from_line(line: &str, keys: &[&str]) -> Self;
}

pub fn to_tsv_string<'a, I: IntoIterator<Item = &'a T>, T: 'a + TsvSerialize>(values: I) -> String {
    let mut result = String::new();

    for key in T::keys() {
        if !result.is_empty() {
            result.push('\t');
        }
        result += key;
    }
    result.push('\n');

    for value in values {
        result += &value.to_line();
        result.push('\n');
    }

    result
}
pub fn from_tsv_string<T: TsvSerialize>(s: String) -> Vec<T> {
    let mut lines = s.lines();
    let keys: Vec<&str> = lines.next().unwrap().split('\t').collect();

    lines.map(|line| T::from_line(line, &keys)).collect()
}

impl<A: TsvValue, B: TsvValue> TsvSerialize for (A, B) {
    #[inline]
    fn keys() -> &'static [&'static str] {
        &["key", "value"]
    }
    fn to_line(&self) -> String {
        format!("{}\t{}", self.0.serialize(), self.1.serialize())
    }
    fn from_line(line: &str, keys: &[&str]) -> Self {
        assert_eq!(keys, Self::keys());
        let mut vals = line.split('\t');

        (
            A::deserialize(vals.next().unwrap()).unwrap(),
            B::deserialize(vals.next().unwrap()).unwrap(),
        )
    }
}

#[macro_export]
macro_rules! impl_tsv {
    (
        $t:ident {
            $(
                $val:ident: $val_type:ty
            ),*
        }
    ) => {
        impl $crate::TsvSerialize for $t {
            fn keys() -> &'static[&'static str] {
                &[
                    $(stringify!($val)),*
                ]
            }
            fn to_line(&self) -> String {
                let mut s = String::new();

                $(
                    s += &<$val_type as $crate::TsvValue>::serialize(&self.$val);
                    s.push('\t');
                )*
                s.pop();

                s
            }
            fn from_line(line: &str, keys: &[&str]) -> Self {
                let mut lines = line.split('\t');
                let mut keys = keys.into_iter();

                $(
                    assert_eq!(keys.next().unwrap(), &stringify!($val));
                    let $val = <$val_type as $crate::TsvValue>::deserialize(&lines.next().unwrap()).unwrap();
                )*

                $t {
                    $($val),*
                }
            }
        }
    };
    (
        $t:ident {
            $(
                $val:ident: $val_type:ty,
            )*
        }
    ) => {
        impl_tsv! {
            $t {
                $(
                    $val: $val_type
                ),*
            }
        }
    };
}
