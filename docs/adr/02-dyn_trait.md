# [ADR-02] custom traits

## Status
Proposed

## Context

As mentioned in the previous ADR ([ADR-01](./02-lifetime.md)), we want to store a vector of object references in an attribute. The decision was to us `Rc` + `RefCell`.

```
struct DataTile<'a'> {
    ...
    entities: Vec<Rc<RefCell<dyn BoardEntity>>>,
    ...
}
```

There is a second part to that, and that is the dynamic type of the referenced objects (`dyn BoardEntity`). What 
this means is that the objects being referenced are instances of **any struct that implements the BoardEntity trait**.

This steems from the idea that we would have different kinds of entities on the board like the player, enemies, items, etc. We would make all of those classes/structs
to implement the BoardEntity so we can store them all together, as we only care about their coordinates.

This implementation works properly until we reach a point where we need to integrate with the Godot API.

Imagine a scenario where we have a struct that derives from a Godot class, e.g. Player

```
#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Player {
    ...
    base: Base<Node2D>,
}
```   

Whether we create an instance from a PackedScene or add a reference to the Player via the Godot editor should not matter and the issue should still be there:

```
#[derive(GodotClass)]
#[class(base=Node)]
struct GameManager {
    base: Base<Node>,
    ...
    player: OnEditor<Gd<Player>>,
    player_ref: Rc<RefCell<Gd<Player>>>, // Notice the Gd<Player> instead of Player   
}

...
impl GameManager {
    ...
    fn ...(&self) {
        let mut player = <get_player_instance> // e.g. self.player_scene.instantiate_as::<Player>();
        let mut player_ref = Rc::new(RefCell::new(player));
        self.player_ref = player_ref;
    }
    ...
}
      
```
It is **very important** to note (as this is part of why there is an issue) that the reference to the player 
(and to any other entity) is `Rc<RefCell<Gd<Player>>>` and not `Rc<RefCell<Player>>`. This is because the
player or any other entities are ultimately Godot objects. I have tried to store the reference to the Player 
without the `Gd<>` type wrapper but the compiler did not like it.

This is what I tried:

```
let mut player_instance = self.player_scene.instantiate_as::<Player>();
let player_obj = player_instance.bind_mut();
let player_objc = *player_obj; // cannot move out of dereference of `GdMut<'_, Player>` move occurs because value has type `Player`, which does not implement the `Copy` trait
let player_ref = Rc::new(RefCell::new(player_objc));
self.player = Some(player_ref);
```
I could try to fix the error but I think that is going down a rabbit hole and probably would make the solution too obtuse.

If we kept going on with the `Rc<RefCell<Gd<Player>>>` and trying to use the dynamic type `dyn BoardEntity` 
we would eventually fail when calling it from within a `#[godot_api]` implementation:
```

fn register_entity_in_data_tile(entity: Rc<RefCell<Gd<dyn BoardEntity>>>) {
    ...
}

```

This would fail to compile because the Gd<T> class does not accept dynamic types as T, in our case `dyn BoardEntity`


## Decision

### Option A
Remove usage of custom traits like BoardEntity.

> we only care about their coordinates

That is a statement done in the context section that might not hold true. We probably want to interact 
with enemies in a different way to items. If that is the case we could have different vectors 
within each DataTile to store different kinds of entities. By doing that, we would not need to use 
dynamic types in the definition of the attributes.


We would go from
```
struct DataTile<'a'> {
    ...
    entities: Vec<Rc<RefCell<dyn BoardEntity>>>,
    ...
}
```
to
```
struct DataTile<'a'> {
    ...
    player: Rc<RefCell<Gd<Player>>>,
    items: Vec<Rc<RefCell<Gd<Item>>>,
    enemies: Vec<Rc<RefCell<Gd<Enemy>>>,
    ...
}
```

UPDATE: This would not properly scale (at least for movement). Currently I have a struct called
MovementManager. It has a reference to the board and has a method for moving BoardEntities around.
It currently receives a boardEntity as a parameter using a dynamic type (`entity: dyn BoardEntity`).

If going this way, I would need to implement as many methods as BoardEntity types to move them around
(e.g. moveItem, moveEnemy, movePlayer) which I would not like doing as they are basically the same 
functionality but I would need to have it duplicate

### Option B
Ditch current approach and go for a more Godot-oriented solution:

Place all entities as children of a single Godot Node whose purpose is to keep track of them. If an
entity is not under said node, it would not exist for the logic.

To check if there is any entites in a certain tile, we would need to go through the whole list of chidren
and check the coordinate for each of them.

Big drawback of this is that it would be much less performant, specially with an elevated number of entities

### Option C

Because gdext (Godot-Rust library) [does not support inheriting from custom Node types](https://godot-rust.github.io/book/godot-api/objects.html#inheritance), the way to go forward is to make BoardEntity an actual user-defined node type.

Player, items, enemies, etc. should by implemented by using components and composition from within. BoardEntity should be dummy and not do anything on its own.

To make a player, we would create a PlayerInputModule and place it as a child of a BoardEntity. The module would
get a reference to the BoardEntity and move it through the board when a certain input is pressed.

If this is done, we could still keep the `entities` field in DataTile, which is preferred over option B.


### Resolution
~~Option A is prefered for now~~
Go with Option C to keep a Vector for entity references

## Consequences
We cannot use Rust dynamic types (e.g. entity: dyn BoardEntity) as parameters while working with Godot as 
they are not supported.

Most likely that implies that we cannot use traits aside from the Godot already-defined ones.
