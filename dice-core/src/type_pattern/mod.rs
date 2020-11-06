use crate::value::Class;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

pub enum TypeOrdering {
    None,
    Sub,
    Self_,
    Super,
}

pub trait TypeOrd<Rhs: ?Sized = Self> {
    fn compare(&self, other: &Rhs) -> TypeOrdering;
}

impl TypeOrd for Class {
    fn compare(&self, other: &Self) -> TypeOrdering {
        if self.instance_type_id() == other.instance_type_id() {
            TypeOrdering::Self_
        } else if self.is_derived_from(other) {
            TypeOrdering::Sub
        } else if other.is_derived_from(self) {
            TypeOrdering::Super
        } else {
            TypeOrdering::None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ParameterType {
    Null,
    Class { class: Class, is_nullable: bool },
}

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
pub struct MultimethodTree<T>
where
    T: Debug,
{
    root: MultimethodNode<T>,
}

impl<T> MultimethodTree<T>
where
    T: Debug,
{
    pub fn insert(&mut self, pattern: MultimethodSignature, value: T) {
        if pattern.parts.len() > 0 {
            let mut current_node = &mut self.root;

            for part in &*pattern.parts {
                if !current_node.children.contains_key(part) {
                    current_node.children.insert(part.clone(), Default::default());
                }

                current_node = current_node.children.get_mut(part).unwrap();
            }

            if current_node.value.is_none() {
                current_node.value = Some(Box::new(value))
            } else {
                todo!("Value already exists error.")
            }
        } else {
            panic!("Pattern is empty.")
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
    value: Option<Box<T>>,
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
fn test() {
    let class1 = Class::new("Class1".into());
    let class2 = Class::new("Class2".into());
    let class3 = Class::new("Class3".into());

    let mut tree = MultimethodTree::<()>::default();
    let sig1 = vec![ParameterType::Class {
        class: class1.clone(),
        is_nullable: false,
    }];

    tree.insert(sig1.into(), ());

    let sig2 = vec![
        ParameterType::Class {
            class: class1.clone(),
            is_nullable: false,
        },
        ParameterType::Class {
            class: class2.clone(),
            is_nullable: false,
        },
    ];

    tree.insert(sig2.into(), ());

    let sig3 = vec![
        ParameterType::Class {
            class: class1.clone(),
            is_nullable: false,
        },
        ParameterType::Class {
            class: class3.clone(),
            is_nullable: false,
        },
    ];

    tree.insert(sig3.into(), ());

    dbg!(tree);
}
