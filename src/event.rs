use crate::Identifier;

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
            phases.push(default_phase());
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
            phases: Vec::new(),
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
        self.register(callback, &default_phase())
    }
    
    pub fn get_phases(&self) -> Vec<Identifier> {
        self.phases.iter().map(|p| p.id.to_owned()).collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ActionResult {
    PASS,
    FAIL,
    SUCCESS,
}

impl ActionResult {
    pub fn should_cancel(&self) -> bool {
        match self {
            Self::PASS => false,
            _ => true,
        }
    }

    pub fn process(&self, input: bool) -> bool {
        match self {
            ActionResult::PASS => input,
            ActionResult::FAIL => false,
            ActionResult::SUCCESS => true,
        }
    }
}

pub fn default_phase() -> Identifier {
    Identifier::new("c", "default_phase")
}

impl Default for ActionResult {
    fn default() -> Self {
        Self::PASS
    }
}

struct PhaseData<I, O> {
    pub id: Identifier,
    pub callbacks: Vec<Box<dyn Fn(I) -> O>>,
}
