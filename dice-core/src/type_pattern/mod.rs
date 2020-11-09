use crate::value::Class;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use wyhash::WyHash;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum TypeOrdering {
    Sub,
    Self_,
    Super,
}

impl TypeOrdering {
    fn is_self_or_sub(&self) -> bool {
        matches!(self, TypeOrdering::Sub | TypeOrdering::Self_)
    }
}

pub trait TypeOrd<Rhs: ?Sized = Self> {
    fn compare(&self, other: &Rhs) -> Option<TypeOrdering>;
}

impl TypeOrd for Class {
    fn compare(&self, other: &Self) -> Option<TypeOrdering> {
        if self.instance_type_id() == other.instance_type_id() {
            Some(TypeOrdering::Self_)
        } else if self.is_derived_from(other) {
            Some(TypeOrdering::Sub)
        } else if other.is_derived_from(self) {
            Some(TypeOrdering::Super)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ParameterType {
    Null,
    Class { class: Class, is_nullable: bool },
}

impl TypeOrd for ParameterType {
    fn compare(&self, other: &Self) -> Option<TypeOrdering> {
        use ParameterType::*;

        match (self, other) {
            (Null, Null) => Some(TypeOrdering::Self_),
            (Null, Class { is_nullable: true, .. }) => Some(TypeOrdering::Sub),
            (Null, Class { is_nullable: false, .. }) => None,
            (Class { is_nullable: true, .. }, Null) => Some(TypeOrdering::Super),
            (Class { is_nullable: false, .. }, Null) => None,
            (
                Class {
                    class: self_class,
                    is_nullable: self_nullable,
                },
                Class {
                    class: other_class,
                    is_nullable: other_nullable,
                },
            ) => match (self_nullable, other_nullable) {
                (false, false) => self_class.compare(other_class),
                (true, true) => self_class.compare(other_class),
                (false, true) => self_class.compare(other_class).and_then(|ord| match ord {
                    TypeOrdering::Sub => Some(TypeOrdering::Sub),
                    TypeOrdering::Self_ => Some(TypeOrdering::Sub),
                    TypeOrdering::Super => None,
                }),
                (true, false) => self_class.compare(other_class).and_then(|ord| match ord {
                    TypeOrdering::Sub => None,
                    TypeOrdering::Self_ => Some(TypeOrdering::Super),
                    TypeOrdering::Super => Some(TypeOrdering::Super),
                }),
            },
        }
    }
}

#[derive(Debug)]
pub struct MultimethodSignature {
    parts: Rc<[ParameterType]>,
}

impl From<Vec<ParameterType>> for MultimethodSignature {
    fn from(signature: Vec<ParameterType>) -> Self {
        Self {
            parts: signature.into(),
        }
    }
}

#[derive(Default, Debug)]
pub struct MultimethodResolver<T>
where
    T: Debug,
{
    root: MultimethodNode<T>,
    cache: HashMap<MultimethodSignature, T, WyHash>,
}

impl<T> MultimethodResolver<T>
where
    T: Debug,
{
    pub fn insert(&mut self, pattern: MultimethodSignature, value: T) {
        let mut current_node = &mut self.root;

        for part in &*pattern.parts {
            if !current_node.children.contains_key(part) {
                current_node.children.insert(part.clone(), Default::default());
            }

            current_node = current_node.children.get_mut(part).unwrap();
        }

        if current_node.value.is_none() {
            current_node.value = Some(value)
        } else {
            todo!("Value already exists error.")
        }
    }

    pub fn find_candidates(&mut self, pattern: MultimethodSignature) -> Vec<&T> {
        // TODO: Find a list of best possible candidates, sorted by most likely match.
        todo!()
    }
}

#[derive(Debug)]
pub struct MultimethodNode<T>
where
    T: Debug,
{
    children: HashMap<ParameterType, MultimethodNode<T>>,
    value: Option<T>,
}

impl<T> Default for MultimethodNode<T>
where
    T: Debug,
{
    fn default() -> Self {
        Self {
            children: Default::default(),
            value: None,
        }
    }
}

#[test]
fn class_ordering() {
    let base = Class::new("Base".into());
    let derived = Class::with_base("Derived".into(), base.clone());
    let unrelated = Class::new("Unrelated".into());

    assert_eq!(base.compare(&derived), Some(TypeOrdering::Super));
    assert_eq!(derived.compare(&base), Some(TypeOrdering::Sub));
    assert_eq!(derived.compare(&derived), Some(TypeOrdering::Self_));
    assert_eq!(base.compare(&base), Some(TypeOrdering::Self_));
    assert_eq!(derived.compare(&unrelated), None);
}

#[test]
fn parameter_type_ordering() {
    let base = Class::new("Base".into());
    let derived = Class::with_base("Derived".into(), base.clone());

    let null = ParameterType::Null;
    let base_non_nullable = ParameterType::Class {
        class: base.clone(),
        is_nullable: false,
    };
    let base_nullable = ParameterType::Class {
        class: base.clone(),
        is_nullable: true,
    };
    let derived_non_nullable = ParameterType::Class {
        class: derived.clone(),
        is_nullable: false,
    };
    let derived_nullable = ParameterType::Class {
        class: derived.clone(),
        is_nullable: true,
    };

    assert_eq!(null.compare(&null), Some(TypeOrdering::Self_));
    assert_eq!(base_non_nullable.compare(&null), None);
    assert_eq!(base_nullable.compare(&null), Some(TypeOrdering::Super));
    assert_eq!(null.compare(&base_nullable), Some(TypeOrdering::Sub));
    assert_eq!(null.compare(&base_non_nullable), None);
    assert_eq!(base_non_nullable.compare(&base_nullable), Some(TypeOrdering::Sub));
    assert_eq!(derived_non_nullable.compare(&base_nullable), Some(TypeOrdering::Sub));
    assert_eq!(derived_nullable.compare(&base_nullable), Some(TypeOrdering::Sub));
    assert_eq!(
        derived_non_nullable.compare(&base_non_nullable),
        Some(TypeOrdering::Sub)
    );
    assert_eq!(derived_nullable.compare(&base_non_nullable), None);
    assert_eq!(derived_non_nullable.compare(&null), None);
    assert_eq!(derived_nullable.compare(&null), Some(TypeOrdering::Super));
}

#[test]
fn multimethod_insert_empty_signature() {
    let mut tree = MultimethodResolver::<()>::default();
    let sig1 = vec![];

    tree.insert(sig1.into(), ());

    assert_eq!(tree.root.value, Some(()))
}
