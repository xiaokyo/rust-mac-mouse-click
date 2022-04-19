use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use core_graphics::geometry::CGPoint;
use std::env;

fn main() {
    let arr: Vec<String> = env::args().collect();
    let x = arr.get(1).expect("x is required");
    let x: f64 = x.parse::<f64>().unwrap();

    let y = arr.get(2).expect("y is required").parse::<f64>().unwrap();

    let ev_left_down_source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();

    let event = CGEvent::new_mouse_event(
        ev_left_down_source,
        CGEventType::LeftMouseDown,
        CGPoint::new(x, y),
        CGMouseButton::Left,
    )
    .unwrap();
    event.post(CGEventTapLocation::HID);

    let ev_left_release_source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();
    let event = CGEvent::new_mouse_event(
        ev_left_release_source,
        CGEventType::LeftMouseUp,
        CGPoint::new(x, y),
        CGMouseButton::Left,
    )
    .unwrap();

    event.post(CGEventTapLocation::HID);
}
