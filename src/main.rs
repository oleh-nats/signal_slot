trait ISignal<Slot: Clone> {
    fn subscribe(&mut self, observer: Box<dyn FnMut(Slot) + 'static>);
    fn unsubscribe(&mut self, observer: *mut ());
    fn notify(&mut self, event: Slot);
}

struct Signal<Event> {
    observers: Vec<Box<dyn FnMut(Event) + 'static>>,
}

impl<Event> Signal<Event> {
    fn new() -> Self {
        Signal { observers: Vec::new() }
    }
}

impl<Event: Clone> ISignal<Event> for Signal<Event> {
    fn subscribe(&mut self, observer: Box<dyn FnMut(Event) + 'static>) {
        self.observers.push(observer);
    }

    fn unsubscribe(&mut self, observer: *mut ()) {
        let index = self.observers.iter().position(|x| {
            let raw: *const () = x.as_ref() as *const _ as *const ();
            raw == observer
        });
        if let Some(index) = index {
            self.observers.remove(index);
        }
    }

    fn notify(&mut self, event: Event) {
        for observer in &mut self.observers {
            observer(event.clone());
        }
    }
}

fn main() {
    let mut observable = Signal::new();

    let mut observer1 = Box::new(|event| println!("Observer 1 received event: {}", event));
    let observer1_ptr = &mut *observer1 as *mut _ as *mut ();
    observable.subscribe(observer1);
    observable.subscribe(Box::new(|event| println!("Observer 2 received event: {}", event)));
    observable.notify("42");
    observable.unsubscribe(observer1_ptr);
    observable.notify("43");
}