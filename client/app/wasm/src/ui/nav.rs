use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

// We set the topics' states by updating the class
// attribute on on their HtmlElements
const TOPIC_PEEK: &'static str = "peek";
const TOPIC_SHOW: &'static str = "show";
const TOPIC_HIDE: &'static str = "hide";

pub struct TopicController {
    topics: Rc<RefCell<Vec<Topic>>>
}

impl TopicController {
    pub fn new() -> TopicController {
        TopicController { topics: Rc::new(RefCell::new(Vec::new())) }
    }

    pub fn add_topic(&mut self, element: HtmlElement) {
        // Get a mutable reference to the topics list
        let mut topics = self.topics.borrow_mut();

        // Clone the topics vector's RC and pass it into the closure
        let idx = topics.len();
        let topics_ref = Rc::clone(&self.topics);
        let interact = Closure::wrap(Box::new(move || {
            // Borrow a reference to the topics vector
            let mut topics_bor = topics_ref.borrow_mut();
            let num = topics_bor.len();

            if topics_bor[idx].is_showing() {
                // If the topic is showing, its content is visible. Hide the content
                // and peek this topic's header along with all the other topics' headers.
                for x in 0..num {
                    topics_bor[x].peek();
                }
            } else {
                // When the content is peeked, only the header is showing. If a 
                // user clicks the header, show the topic (i.e. expand the content
                // under the header). All the other topics' headers should be hidden.  
                for x in 0..num {
                    if x == idx {
                        topics_bor[x].show();
                    } else {
                        topics_bor[x].hide();
                    }
                }
            }
        }) as Box<FnMut()>);
        element.set_onclick(Some(interact.as_ref().unchecked_ref()));
        interact.forget();

        // Create the new topic and set click listener for the topic
        let topic = Topic::new(element);
        topics.push(topic);
    }
}

struct Topic {
    container: HtmlElement,
    showing: bool
}

impl Topic {

    pub fn new(element: HtmlElement) -> Topic {
        Topic { container: element, showing: false}
    }

    pub fn is_showing(&self) -> bool {
        self.showing
    }

    // Show only the header for this topic
    pub fn peek(&mut self) {
        self.container.set_class_name(TOPIC_PEEK);
        self.showing = false;
    }

    // Reveal the content for this topic
    pub fn show(&mut self) {
        self.container.set_class_name(TOPIC_SHOW);
        self.showing = true;
    }
    
    // Hide everything that has to do with this topic
    pub fn hide(&mut self) {
        self.container.set_class_name(TOPIC_HIDE);
        self.showing = false;
    }
}