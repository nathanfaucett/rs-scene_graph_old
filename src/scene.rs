use collections::vec::Vec;
use collections::btree_map::BTreeMap;
use collections::boxed::Box;
use alloc::arc::Arc;
use core::cell::RefCell;

use id::Id;
use entity::Entity;
use component::Component;
use component_manager::ComponentManager;


struct SceneData {
    initted: bool,
    entities: Vec<Entity>,
    component_managers_initted: BTreeMap<Id, bool>,
    component_managers: BTreeMap<Id, Box<ComponentManager>>,
}

#[derive(Clone)]
pub struct Scene {
    data: Arc<RefCell<SceneData>>,
}

impl Scene {

    pub fn new() -> Self {
        Scene {
            data: Arc::new(RefCell::new(SceneData {
                initted: false,
                entities: Vec::new(),
                component_managers_initted: BTreeMap::new(),
                component_managers: BTreeMap::new(),
            }))
        }
    }

    pub fn init(&self) -> &Self {
        if !self.data.borrow().initted {
            let ref mut component_managers_initted = self.data.borrow_mut().component_managers_initted;

            for (id, component_manager) in self.data.borrow().component_managers.iter() {
                if !component_managers_initted.contains_key(id) {
                    component_managers_initted.insert(id.clone(), true);
                    component_manager.init();
                }
            }
        }
        self
    }

    pub fn update(&self) -> &Self {
        for (_, component_manager) in self.data.borrow().component_managers.iter() {
            component_manager.update();
        }
        self
    }

    pub fn add_entity(&self, entity: Entity) -> &Self {
        if let Some(scene) = entity.scene() {
            if scene != *self {
                scene.remove_entity(entity.clone());
            } else {
                return self;
            }
        }

        self.data.borrow_mut().entities.push(entity.clone());
        entity.__set_scene(self.clone());

        self
    }
    pub fn has_entity(&self, entity: Entity) -> bool {
        match self.data.borrow().entities.iter().position(|e| *e == entity) {
            Some(_) => true,
            None => false,
        }
    }
    pub fn remove_entity(&self, entity: Entity) -> &Self {
        if self.__remove_entity(entity.clone()) {
            entity.__remove_scene(self.clone());
        }
        self
    }

    pub fn __remove_entity(&self, entity: Entity) -> bool {
        let removed;
        {
            let ref mut entities = self.data.borrow_mut().entities;

            match entities.iter().position(|e| *e == entity) {
                Some(i) => {
                    entities.remove(i);
                    removed = true;
                },
                None => {
                    removed = false
                },
            }
        }

        if removed {
            entity.__remove_scene_children(self.clone());
        }

        removed
    }

    pub fn __add_component(&self, component: &Box<Component>) {
        let id = component.component_manager_id();

        if !self.data.borrow().component_managers.contains_key(&id) {
            let component_manager = component.component_manager();
            self.data.borrow_mut().component_managers.insert(id, component_manager);
        }

        self.data.borrow().component_managers.get(&id).unwrap().add_component(component);

        if self.data.borrow().initted {
            if !self.data.borrow().component_managers_initted.contains_key(&id) {
                self.data.borrow_mut().component_managers_initted.insert(id, true);
            }
            self.data.borrow().component_managers.get(&id).unwrap().init();
        }
    }
    pub fn __remove_component(&self, component: &Box<Component>) {
        let id = component.component_manager_id();
        let is_empty;

        {
            let ref component_managers = self.data.borrow().component_managers;
            let component_manager = component_managers.get(&id).unwrap();
            is_empty = component_manager.is_empty();
            component_manager.remove_component(component);
        }

        if is_empty {
            {
                let ref mut component_managers = self.data.borrow_mut().component_managers;
                component_managers.get(&id).unwrap().destroy();
                component_managers.remove(&id);
            }
            if !self.data.borrow().component_managers_initted.contains_key(&id) {
                self.data.borrow_mut().component_managers_initted.insert(id, true);
            }
        }
    }
}

impl PartialEq<Scene> for Scene {
    fn eq(&self, other: &Scene) -> bool {
        (&*self.data.borrow() as *const _) == (&*other.data.borrow() as *const _)
    }
    fn ne(&self, other: &Scene) -> bool {
        !self.eq(other)
    }
}
