use crate::{parsers::method::Method, JavaClassFile};

#[derive(Debug)]
pub enum QueryResult {
    QMethod(Method),
    QMethodList(Vec<Method>),
}

#[derive(Debug)]
pub enum QueryType {
    QMethod(String),
    QMethodList,
}

pub struct Query<'a> {
    class_file: &'a JavaClassFile,
}

impl<'a> Query<'a> {
    pub fn new(class_file: &'a JavaClassFile) -> Self {
        Self { class_file }
    }

    pub fn query(&self, q: QueryType) -> Option<QueryResult> {
        match q {
            QueryType::QMethod(name) => {
                let m = self.class_file.methods.iter().find(|m| m.name == name);
                if m.is_some() {
                    let m = m.unwrap();
                    return Some(QueryResult::QMethod(Method {
                        access_flags: m.access_flags.clone(),
                        name: m.name.clone(),
                        descriptor: m.descriptor.clone(),
                        attrs: vec![],
                    }));
                }
                None
            }
            QueryType::QMethodList => {
                let list = self
                    .class_file
                    .methods
                    .iter()
                    .map(|m| {
                        return Method {
                            access_flags: m.access_flags.clone(),
                            name: m.name.clone(),
                            descriptor: m.descriptor.clone(),
                            attrs: vec![],
                        };
                    })
                    .collect::<Vec<Method>>();
                return Some(QueryResult::QMethodList(list));
            }
        }
    }
}
