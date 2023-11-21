use bevy::{prelude::*, window::{PrimaryWindow, WindowResized}, ecs::{schedule::ScheduleLabel, system::Command}};
use bevy_debug_text_overlay::screen_print;

pub struct UIElementPlugin;

impl Plugin for UIElementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ParentResizedEvent>()
            .add_event::<RecalculateSizeEvent>()
            .insert_resource(ToBeResized{ elements: vec![] })
            .add_schedule(PreUILayout, Schedule::new())
            .add_schedule(Sizing, Schedule::new())
            .add_schedule(UILayout, Schedule::new())
            .add_schedule(PostUILayout, Schedule::new())
            .add_systems(Update, run_ui_schedule)
            .add_systems(PreUILayout, dispatch_resizes)
            .add_systems(UILayout, window_resize_update)
            .add_systems(UILayout, resize_new_root_elements)
            .add_systems(UILayout, basic_children_layout_update)
            .add_systems(UILayout, save_resize)
            .add_systems(UILayout, resize_new_parent)
            .add_systems(UILayout, catch_recalculate_size.after(resize_new_parent))
            .add_systems(Sizing, calculate_dynamic_size);
    }
}

//SCHEDULES
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PreUILayout;
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Sizing;
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct UILayout;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PostUILayout;

fn run_ui_schedule(world : &mut World)
{
    let mut do_layouts = true;

    while do_layouts {
        
        world.run_schedule(PreUILayout);
        world.run_schedule(Sizing);
        world.run_schedule(UILayout);
        world.run_schedule(PostUILayout);
        
        let to_be_resized = world.remove_resource::<ToBeResized>().expect("This resource should be present");
        do_layouts = to_be_resized.elements.len() > 0;
        world.insert_resource(to_be_resized);
    }
}

#[derive(Resource)]
pub struct ToBeResized {
    pub elements : Vec<ParentResizedEvent>
}

#[derive(Event, Clone, Debug)]
pub struct ParentResizedEvent {
    pub entity : Entity,
    pub size : Vec2,
    pub offset : Vec2,
}

#[derive(Event)]
pub struct RecalculateSizeEvent {
    pub entity : Entity
}

fn dispatch_resizes(
    mut to_be_resized : ResMut<ToBeResized>,
    mut resize_ev : EventWriter<ParentResizedEvent>,
) {
    if to_be_resized.elements.len() > 0 {
        info!("Dispatching {} resizes", to_be_resized.elements.len());
    }

    while to_be_resized.elements.len() > 0 {
        resize_ev.send(to_be_resized.elements.remove(0));
    }
}

#[derive(Component)]
pub struct UIElement {
    is_new : bool,
    last_resize : Option<ParentResizedEvent>,
}

impl Default for UIElement {
    fn default() -> Self {
        Self {
            is_new: true,
            last_resize: None
        }
    }
}

fn window_resize_update(
    q_window : Query<&Window, With<PrimaryWindow>>,
    mut resize_ev : EventReader<WindowResized>,
    q_ui_elements : Query<Entity, (With<UIElement>, Without<Window>, Without<Parent>)>,
    mut to_be_resized : ResMut<ToBeResized>,
) {
    for resize in resize_ev.iter() {
        if let Ok(window) = q_window.get(resize.window) {
            for ui_elem in q_ui_elements.iter() {
                to_be_resized.elements.push(ParentResizedEvent { 
                    entity: ui_elem, 
                    size: Vec2 { x: window.width(), y: window.height()}, 
                    offset: Vec2 { x: window.width() / -2.0, y: window.height() / -2.0}
                });
            }   
        }
    } 
}

fn resize_new_root_elements(
    mut q_ui_elements : Query<(Entity, &mut UIElement), Without<Parent>>,
    q_window : Query<&Window, With<PrimaryWindow>>,
    mut to_be_resized : ResMut<ToBeResized>,
) {
    let window = q_window.single();

    for (entity, mut elem) in q_ui_elements.iter_mut() {
        if elem.is_new {
            to_be_resized.elements.push(ParentResizedEvent { 
                entity: entity, 
                size: Vec2 { x: window.width(), y: window.height()}, 
                offset: Vec2 { x: window.width() / -2.0, y: window.height() / -2.0}
            });

            elem.is_new = false;
        }
    }
}

/*
This ensures that when a new UIElement child is added to an element, it will dispatch 
an event to resize the parent so that it lays out the children correctly
 */
fn resize_new_parent(
    mut q_ui_elements : Query<(&mut UIElement, &Parent)>,
    mut recalculate_ev : EventWriter<RecalculateSizeEvent>,
) {
    for (mut elem, parent) in q_ui_elements.iter_mut() {
        if elem.is_new {

            recalculate_ev.send(RecalculateSizeEvent { entity: parent.get() });

            elem.is_new = false;
        }
    }
}

/*
This is the second step of the above system, which makes the parent redo its layout
*/
fn catch_recalculate_size(
    mut q_ui_elements : Query<&UIElement>,
    mut recalculate_ev : EventReader<RecalculateSizeEvent>,
    mut to_be_resized : ResMut<ToBeResized>,
) {
    for recalc_ev in recalculate_ev.iter() {
        if let Ok(elem) = q_ui_elements.get(recalc_ev.entity) {
            if let Some(last_resize) = &elem.last_resize {
                to_be_resized.elements.push(last_resize.clone());
            }
        }
    }
}

fn save_resize(
    mut q_ui_elements : Query<&mut UIElement>,
    mut resize_ev : EventReader<ParentResizedEvent>,
) {
    for resize in resize_ev.iter() {
        if let Ok(mut ui_elem) = q_ui_elements.get_mut(resize.entity) {
            ui_elem.last_resize = Some(resize.clone());
        }
    }
}


#[derive(Component)]
pub struct BasicChildrenLayout;

fn basic_children_layout_update(
    mut resize_ev : EventReader<ParentResizedEvent>,
    q_cascade : Query<&Children, With<BasicChildrenLayout>>,
    mut to_be_resized : ResMut<ToBeResized>
) {
    for layout in resize_ev.iter() {
        if let Ok(children) = q_cascade.get(layout.entity) {
            for child in children.iter() {
                to_be_resized.elements.push(ParentResizedEvent { 
                    entity: *child, 
                    size: layout.size, 
                    offset: layout.offset 
                });
            }
        }
    }
}

#[derive(Bundle)]
pub struct SizeBundle {
    sized : Sized,
    dynamic_size : DynamicSize
}

impl Default for SizeBundle {
    fn default() -> Self {
        Self { sized: Default::default(), dynamic_size: Default::default() }
    }
}

impl SizeBundle {
    pub fn from_constraints(x : SizeConstraint, y : SizeConstraint) -> SizeBundle {
        Self { sized: default(), dynamic_size: DynamicSize { x, y } }
    }
}

#[derive(Component)]
pub struct Sized {
    pub size : Vec2,
}

impl Default for Sized {
    fn default() -> Self {
        Self { size: Default::default() }
    }
}

#[derive(Component)]
pub struct DynamicSize {
    pub x : SizeConstraint,
    pub y : SizeConstraint,
}

impl Default for DynamicSize {
    fn default() -> Self {
        Self { x: SizeConstraint::MatchParent, y: SizeConstraint::MatchParent }
    }
}

impl DynamicSize {
    pub fn calculate(&self, parent_size : Vec2) -> Vec2 {
        Vec2 { 
            x: self.x.calculate(parent_size.x), 
            y: self.y.calculate(parent_size.y) 
        }
    }
}

pub enum SizeConstraint {
    Const(f32),
    Percent(f32),
    Min(f32, Box<SizeConstraint>),
    Max(f32, Box<SizeConstraint>),
    MatchParent,
}

impl SizeConstraint {
    pub fn calculate(&self, parent_val : f32) -> f32 {
        match self {
            SizeConstraint::Const(c) => *c,
            SizeConstraint::Percent(pc) => *pc * parent_val,
            SizeConstraint::Min(val, size) => val.max(size.calculate(parent_val)),
            SizeConstraint::Max(val, size) => val.min(size.calculate(parent_val)),
            SizeConstraint::MatchParent => parent_val,
        }
    }
}

pub fn calculate_dynamic_size(
    mut q_sized : Query<(&mut Sized, &DynamicSize)>,
    mut resized_ev : EventReader<ParentResizedEvent>,
) {
    for resized in resized_ev.iter() {
        if let Ok((mut sized, constraints)) = q_sized.get_mut(resized.entity) {
            sized.size = constraints.calculate(resized.size);
        }
    }
}