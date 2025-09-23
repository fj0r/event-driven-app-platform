# event-driven-app-platform

```nu
use __.nu
__ serve --rpk
```

or

gateway
```nu
use __.nu
__ rpk up
__ gw up
```

ui
```nu
use __.nu
__ ui up
```


## chat
```nu
use __.nu
__ pg up
__ pg migrate
__ chat up
```

# Design

## message broker

## control center
In the `chat/src/libs/logic` directory, there are several control centers (business logic is implemented here, and they are separate services, so they're called control centers).

`echo` is the simplest one - it echoes back whatever you send, similar in nature to hello world.

`chat` is like slack-style chat, divided by channels.

`crm` is for AI customer service - both users chatting and backend customer service agents can see it (if they need to grab orders), and it assigns users to one or more agents.

Users see a single chat box, while customer service sees multiple channels, so `chat` needs to be implemented first.

## UI
### menu
Currently, components basically bind to a single value, for example, a text input box corresponds to a string "xxx"
For menus, there are two values: one is all the candidate options, and the other is the already selected value
Previously, I thought about putting the candidate options in children, but it's not convenient for dynamic binding, and defining styles is also cumbersome (normally define an item value as the style for all child items, rather than wrapping all child items with styles and copying them N times (referencing the rack component design)

If a component can bind to multiple values, one approach is to add an additional field on top of the (existing) value field. But this can get messy and inconsistent
Another approach is to refactor the value field to accept multiple values. But this involves more changes (this shows the advantage of Rust; if it were JavaScript, I wouldn't even dare to imagine how painful it would be), and additionally, you need to declare extra fields when binding (but you can specify a default value)
