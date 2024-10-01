use std::cell::RefCell;
use std::rc::Rc;

// 观察者接口
trait Observer {
    fn update(&self, message: &str);
}

// 主题接口
trait Subject {
    fn attach(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn detach(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn notify(&self, message: &str);
}

// 具体主题
struct ConcreteSubject {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
}

impl ConcreteSubject {
    fn new() -> Self {
        ConcreteSubject {
            observers: Vec::new(),
        }
    }
}

impl Subject for ConcreteSubject {
    fn attach(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn detach(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.retain(|o| !Rc::ptr_eq(o, &observer));
    }

    fn notify(&self, message: &str) {
        for observer in &self.observers {
            observer.borrow().update(message);
        }
    }
}

// 具体观察者
struct ConcreteObserver {
    name: String,
}

impl Observer for ConcreteObserver {
    fn update(&self, message: &str) {
        println!("{} received message: {}", self.name, message);
    }
}


#[cfg(test)]
mod observer_test {
    use super::*;


    #[test]
    fn test_01() {
        let mut subject = ConcreteSubject::new();

        let observer1 = Rc::new(RefCell::new(ConcreteObserver {
            name: String::from("Observer 1"),
        }));
        let observer2 = Rc::new(RefCell::new(ConcreteObserver {
            name: String::from("Observer 2"),
        }));

        subject.attach(observer1.clone());
        subject.attach(observer2.clone());

        subject.notify("Hello Observers!");

        subject.detach(observer1);
        subject.notify("Observer 1 has been detached.");
    }
}


