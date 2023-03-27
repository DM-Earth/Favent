pub mod event;
#[cfg(test)]
mod tests {
    use super::event::*;

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum ActionResult {
        PASS,
        FAIL,
        SUCCESS,
    }

    impl Default for ActionResult {
        fn default() -> Self {
            Self::PASS
        }
    }

    #[test]
    fn event_test() {
        let mut event = Event::new_default(Box::new(event_invoker));
        event.register_default(Box::new(func));
        event.register(Box::new(func2), &phase::default_phase());
        println!("{:?}", event.invoke("fuckumc"));
    }

    fn event_invoker<'a>(
        callbacks: Vec<&dyn Fn(&'a str) -> ActionResult>,
        input: &'a str,
    ) -> ActionResult {
        for callback in callbacks {
            let result = callback(input);
            match &result {
                ActionResult::PASS => continue,
                _ => (),
            }

            return result;
        }
        ActionResult::PASS
    }

    fn func(s: &str) -> ActionResult {
        if s.contains("fuck") {
            ActionResult::FAIL
        } else if s.contains("dm") {
            ActionResult::SUCCESS
        } else {
            ActionResult::PASS
        }
    }

    fn func2(s: &str) -> ActionResult {
        if s.contains("mn") {
            ActionResult::FAIL
        } else if s.contains("mc") {
            ActionResult::SUCCESS
        } else {
            ActionResult::PASS
        }
    }
}
