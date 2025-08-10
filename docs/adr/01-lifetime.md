# [ADR-01] Lifetime annotations with Godot

## Status
Accepted

## Context

At the time of writing this, DataTile define an `entities` field as a 
Vector of references to BoardEntities.

```
struct DataTile<'a'> {
      ...
    entities: Vec<&'a dyn BoardEntity>,
    ...
}
```

This is to keep track of all the entities there are in a specific tile. The type 
anotation of this attribute requires a lifetime to specify how long the references live.

That is fine if we just work with Rust structs, but GodotClasses
and and thus Rust structs derived from them **do NOT support lifetime
annotation**.

This causes that in the DataTile struct we can use a user-defined lifetime
(`<'a'>`) but as it is propragated and eventually a DataTile is used
from within a GodotClass to add an entity to the `entities` vector
within DataTile, we will have to leverage the `'static'` lifetime.

This ends up making the code imposible to expand as Rust will constantly
complain that a variable/attribute/parameter lifetime will not live
enough as it should match `static`.

As of now I am going to leave the Board with its data field with
lifetime annotation. If it causes the whole code to became unworkable in
the future I will have to get rid of any code/field that requires
lifetimes, most likely I will need to only have structs that derive from
GodotClasses.


## Decision
After doing some investigation, the solution seems to be to leverage the native Rust structs
`Rc` and `RefCell`.

`Rc` takes care of the Lifetime and allows us not having to define or use any lifetime annotation.

`RefCell` allows us to use the object references as mutable, as by default, references obtained from an `Rc` instance are not mutable.

As an example, we would be going from:
```
struct DataTile<'a'> {
      ...
    entities: Vec<&'a dyn BoardEntity>,
    ...
}
```

to:

```
struct DataTile<'a'> {
      ...
    entities: Vec<Rc<RefCell<dyn BoardEntity>>>,
    ...
}
```

## Consequences
We should not use lifetime annotations. When needed, use `Rc` + `Refcell`.

