# Favent

This is an QSL/FAPI events inspired event system for Rust.

## Usage

Event is a place storing callbacks for this event with phases and an invoker with a default implementation. To create an event, use `new` or `new_default` functions in the `Event` struct.

Please check the `test` module for examples.

### Callbacks

You need callbacks to listen for events. When registering a callback into an event, you need to provide a phase id for implementing ordering.

### Phases

Favent use phases to order callbacks. You need to order the phases when creating an event.
