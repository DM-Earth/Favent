use self::phase::{Identifier, PhaseData};

pub struct Event<I, O> {
    phases: Vec<PhaseData<I, O>>,
    invoker: Box<dyn Fn(Vec<&dyn Fn(I) -> O>, I) -> O>,
    default_impl: Box<dyn Fn(I) -> O>,
}

impl<I, O> Event<I, O> {
    pub fn new<C, V>(invoker: Box<V>, empty_impl: Box<C>, mut phases: Vec<Identifier>) -> Self
    where
        V: Fn(Vec<&dyn Fn(I) -> O>, I) -> O + 'static,
        C: Fn(I) -> O + 'static,
    {
        if phases.is_empty() {
            phases.push(self::phase::default_phase());
        }

        Self {
            phases: phases
                .iter()
                .map(|id| PhaseData {
                    id: id.clone(),
                    callbacks: Vec::new(),
                })
                .collect(),
            invoker,
            default_impl: empty_impl,
        }
    }

    pub fn new_default<V>(invoker: Box<V>) -> Self
    where
        V: Fn(Vec<&dyn Fn(I) -> O>, I) -> O + 'static,
        O: Default,
    {
        Self {
            phases: vec![self::phase::PhaseData {
                id: self::phase::default_phase(),
                callbacks: Vec::new(),
            }],
            invoker,
            default_impl: Box::new(|_i| O::default()),
        }
    }

    pub fn invoke(&self, input: I) -> O {
        if self.phases.is_empty() {
            self.default_impl.as_ref()(input)
        } else {
            self.invoker.as_ref()(
                {
                    let mut vec = Vec::new();
                    for phase in &self.phases {
                        vec.append(&mut phase.callbacks.iter().map(|b| b.as_ref()).collect());
                    }
                    vec
                },
                input,
            )
        }
    }

    pub fn register<T>(&mut self, callback: Box<T>, phase: &Identifier) -> bool
    where
        T: Fn(I) -> O + 'static,
    {
        match self.phases.iter_mut().find(|p| p.id.eq(phase)) {
            Some(phase) => {
                phase.callbacks.push(callback);
                true
            }
            None => false,
        }
    }

    pub fn register_default<T>(&mut self, callback: Box<T>) -> bool
    where
        T: Fn(I) -> O + 'static,
    {
        self.register(callback, &self::phase::default_phase())
    }

    pub fn get_phases(&self) -> Vec<Identifier> {
        self.phases.iter().map(|p| p.id.to_owned()).collect()
    }
}

pub mod phase {
    use std::fmt::Debug;

    #[derive(Debug, PartialEq, Clone)]
    pub struct Identifier {
        namespace: String,
        path: String,
    }

    impl Identifier {
        pub const NAMESPACE_SEPARATOR: char = ':';

        pub fn new(namespace: &str, path: &str) -> Self {
            Self {
                namespace: namespace.to_owned(),
                path: path.to_string(),
            }
        }

        pub fn from(id: String) -> Option<Self> {
            let s = id.split_once(':')?;
            Some(Self::new(s.0, s.1))
        }

        pub fn get_namespace(&self) -> &str {
            &self.namespace
        }

        pub fn get_path(&self) -> &str {
            &self.path
        }
    }

    impl ToString for Identifier {
        fn to_string(&self) -> String {
            format!("{}:{}", self.namespace, self.path)
        }
    }

    pub(super) struct PhaseData<I, O> {
        pub id: Identifier,
        pub callbacks: Vec<Box<dyn Fn(I) -> O>>,
    }

    pub fn default_phase() -> Identifier {
        Identifier::new("c", "default_phase")
    }
}
