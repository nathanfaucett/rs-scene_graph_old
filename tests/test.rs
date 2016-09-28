#![no_std]
#![feature(collections)]


extern crate collections;

extern crate shared;

extern crate scene_graph;

use collections::string::String;
use collections::string::ToString;
use collections::vec::Vec;
use collections::boxed::Box;

use shared::Shared;

use scene_graph::{Scene, Entity, Component, ComponentManager, Id};


struct SomeComponentManagerData {
    scene: Option<Scene>,
    components: Vec<SomeComponent>,
}
#[derive(Clone)]
pub struct SomeComponentManager {
    data: Shared<SomeComponentManagerData>,
}
impl SomeComponentManager {
    fn new() -> SomeComponentManager {
        SomeComponentManager {
            data: Shared::new(SomeComponentManagerData {
                scene: None,
                components: Vec::new(),
            })
        }
    }
}
impl ComponentManager for SomeComponentManager {

    fn get_id(&self) -> Id { Id::of::<SomeComponentManager>() }

    fn get_scene(&self) -> Option<Scene> {
        match self.data.scene {
            Some(ref scene) => Some(scene.clone()),
            None => None,
        }
    }
    fn set_scene(&mut self, scene: Option<Scene>) {
        self.data.scene = scene;
    }

    fn get_order(&self) -> usize { 0 }
    fn is_empty(&self) -> bool {
        self.data.components.len() == 0
    }

    fn clear(&mut self) {}
    fn init(&mut self) {}
    fn update(&mut self) {}

    fn add_component(&mut self, component: &Box<Component>) {
        let component = component.downcast_ref::<SomeComponent>().unwrap();
        self.data.components.push(component.clone());
    }
    fn remove_component(&mut self, component: &Box<Component>) {
        let component = component.downcast_ref::<SomeComponent>().unwrap();
        let ref mut components = self.data.components;

        match components.iter().position(|c| *c == *component) {
            Some(i) => {
                components.remove(i);
            },
            None => (),
        }
    }
}

struct SomeComponentData {
    entity: Option<Entity>,
}
#[derive(Clone)]
pub struct SomeComponent {
    data: Shared<SomeComponentData>,
}
impl SomeComponent {
    pub fn new() -> Self {
        SomeComponent {
            data: Shared::new(SomeComponentData {
                entity: None,
            })
        }
    }
    pub fn hello(&self) -> String {
        "Hello, world!".to_string()
    }
}
impl Component for SomeComponent {

    fn get_id(&self) -> Id { Id::of::<SomeComponent>() }

    fn new_component_manager(&self) -> Box<ComponentManager> {
        Box::new(SomeComponentManager::new())
    }
    fn get_component_manager_id(&self) -> Id {
        Id::of::<SomeComponentManager>()
    }
    fn get_entity(&self) -> Option<Entity> {
        self.data.entity.clone()
    }
    fn set_entity(&mut self, entity: Option<Entity>) {
        self.data.entity = entity;
    }
}
impl PartialEq<SomeComponent> for SomeComponent {
    fn eq(&self, other: &SomeComponent) -> bool {
        (&*self.data as *const _) == (&*other.data as *const _)
    }
    fn ne(&self, other: &SomeComponent) -> bool {
        !self.eq(other)
    }
}


#[test]
fn test_scene() {
    let mut scene = Scene::new();
    let mut grandparent = Entity::new();
    let mut parent = Entity::new();
    let mut child = Entity::new();

    grandparent.add_component(SomeComponent::new());
    parent.add_component(SomeComponent::new());
    child.add_component(SomeComponent::new());

    parent.add_child(&mut child);
    grandparent.add_child(&mut parent);

    scene.add_entity(&mut grandparent);

    scene.init();

    assert_eq!(grandparent.has_component::<SomeComponent>(), true);
    assert_eq!(parent.has_component::<SomeComponent>(), true);
    assert_eq!(child.has_component::<SomeComponent>(), true);

    let some_component = grandparent.get_component::<SomeComponent>().unwrap();
    assert_eq!(some_component.hello(), "Hello, world!".to_string());

    assert_eq!(scene.has_entity(&grandparent), true);
    assert_eq!(scene.has_entity(&parent), true);
    assert_eq!(scene.has_entity(&child), true);

    grandparent.remove_component::<SomeComponent>();
    parent.remove_component::<SomeComponent>();
    child.remove_component::<SomeComponent>();

    scene.remove_entity(&mut child);

    assert_eq!(parent.has_child(&child), false);
    assert_eq!(scene.has_entity(&child), false);

    scene.remove_entity(&mut grandparent);

    assert_eq!(scene.has_entity(&grandparent), false);
    assert_eq!(scene.has_entity(&parent), false);
    assert_eq!(grandparent.has_child(&parent), true);

    scene.clear();
}
