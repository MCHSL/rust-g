use std::{
    cell::RefCell,
    collections::hash_map::{Entry, HashMap},
    time::Instant,
};

thread_local!( static INSTANTS: RefCell<HashMap<String, Instant>> = RefCell::new(HashMap::new()) );

byond_fn! { time_microseconds(instant_id) {
    INSTANTS.with(|instants| {
        let mut map = instants.borrow_mut();
        let instant = match map.entry(instant_id.into()) {
            Entry::Occupied(elem) => elem.into_mut(),
            Entry::Vacant(elem) => elem.insert(Instant::now()),
        };
        Some(instant.elapsed().as_micros().to_string())
    })
} }

byond_fn! { time_milliseconds(instant_id) {
    INSTANTS.with(|instants| {
        let mut map = instants.borrow_mut();
        let instant = match map.entry(instant_id.into()) {
            Entry::Occupied(elem) => elem.into_mut(),
            Entry::Vacant(elem) => elem.insert(Instant::now()),
        };
        Some(instant.elapsed().as_millis().to_string())
    })
} }

byond_fn! { time_reset(instant_id) {
    INSTANTS.with(|instants| {
        let mut map = instants.borrow_mut();
        map.insert(instant_id.into(), Instant::now());
        Some("")
    })
} }

