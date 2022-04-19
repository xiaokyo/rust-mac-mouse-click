use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGMouseButton};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use core_graphics::geometry::CGPoint;
use std::env;

// 鼠标按键枚举
enum MouseEvent {
    Left,
    Right,
}

enum MouseAction {
    Press,
    Release,
}

/**
 * 发送鼠标事件
 */
fn send_mouse_event(_type: MouseEvent, _action: MouseAction, x: f64, y: f64) {
    let ev_left_down_source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();

    let (mut event_type, mouse_button) = match _type {
        MouseEvent::Left => (CGEventType::LeftMouseDown, CGMouseButton::Left),
        MouseEvent::Right => (CGEventType::RightMouseDown, CGMouseButton::Right),
    };

    if let MouseAction::Release = _action {
        // 如果是松开就替换松开的event_type
        event_type = match _type {
            MouseEvent::Left => CGEventType::LeftMouseUp,
            MouseEvent::Right => CGEventType::RightMouseUp,
        };
    }

    let event = CGEvent::new_mouse_event(
        ev_left_down_source,
        event_type,
        CGPoint::new(x, y),
        mouse_button,
    )
    .unwrap();
    event.post(CGEventTapLocation::HID);
}

fn main() {
    let arr: Vec<String> = env::args().collect();
    let x = arr.get(1).expect("x is required");
    let x: f64 = x.parse::<f64>().unwrap();

    let y = arr.get(2).expect("y is required").parse::<f64>().unwrap();

    send_mouse_event(MouseEvent::Left, MouseAction::Press, x, y);
    send_mouse_event(MouseEvent::Left, MouseAction::Release, x, y);
}
