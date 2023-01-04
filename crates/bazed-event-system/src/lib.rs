#![forbid(unreachable_pub)]
#![allow(rustdoc::private_intra_doc_links)]

use std::collections::HashMap;

use uuid::Uuid;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct HandlerId(pub Uuid);

/// Event types uniquely identify the kind of an event.
/// TODO we'll need decide on some way to namespace these or something.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct EventType(String);

/// An event, represented as unstructured json.
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Event {
    data: serde_json::Value,
}

pub type EventCallback = Box<dyn FnMut(&Event) + 'static>;

pub struct EventListener {
    callback: EventCallback,
}

impl EventListener {
    fn call(&mut self, event: &Event) {
        (self.callback)(event)
    }
}

pub struct EventSystem {
    event_listeners: HashMap<EventType, HashMap<HandlerId, EventListener>>,
}

impl EventSystem {
    pub fn register_event_listener(
        &mut self,
        typ: EventType,
        listener: EventListener,
    ) -> HandlerId {
        let handler_id = HandlerId(Uuid::new_v4());
        self.event_listeners
            .entry(typ)
            .or_default()
            .insert(handler_id.clone(), listener);
        handler_id
    }

    pub fn unregister_event_listener(&mut self, handler_id: &HandlerId) {
        // TODO optimize this, this is O(n) where n is number of event types, lmao
        for handlers in self.event_listeners.values_mut() {
            handlers.remove(handler_id);
        }
    }

    pub fn broadcast_event(&mut self, typ: &EventType, event: &Event) {
        if let Some(listeners) = self.event_listeners.get_mut(&typ) {
            for listener in listeners.values_mut() {
                listener.call(&event)
            }
        }
    }
}
