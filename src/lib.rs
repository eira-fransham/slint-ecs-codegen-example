use std::mem;

use bevy_ecs::{
    component::Component,
    entity::{Entity, EntityHashSet},
    event::{EntityEvent, Event},
    hierarchy::Children,
    name::Name,
    observer::On,
    query::With,
    system::{Commands, In, IntoSystem, Query, SystemId},
    template::{FnTemplate, FromTemplate, TemplateContext, template},
};
use bevy_scene::{Scene, SceneComponent, bsn, on};
use i_slint_core::{
    Brush, SharedString,
    graphics::{Point, Size},
    lengths::{LogicalEdges, LogicalLength},
};

// --------------------------------------------------
// Library code
// --------------------------------------------------

#[derive(Component)]
#[relationship(relationship_target = Descendants)]
pub struct Root(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = Root)]
pub struct Descendants(EntityHashSet);

// TODO: Needs bevy-n2m
#[derive(Component, FromTemplate)]
pub struct Binding(pub Entity);

#[derive(Component, Clone, Default)]
pub struct Value<T>(pub T);

#[derive(Component, Clone)]
pub struct ValueExpr<T>(pub SystemId<(), T>);

#[derive(Component)]
#[relationship(relationship_target = Properties)]
pub struct PropertyOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = PropertyOf)]
pub struct Properties(EntityHashSet);

#[derive(Component)]
#[relationship(relationship_target = ModelElements)]
pub struct ModelElementOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = ModelElementOf)]
pub struct ModelElements(Vec<Entity>);

pub fn value_expr<O, S, M>(
    system: S,
) -> FnTemplate<
    impl Fn(&mut TemplateContext<'_, '_>) -> bevy_ecs::error::Result<ValueExpr<O>> + Clone,
    ValueExpr<O>,
>
where
    S: IntoSystem<In<Entity>, O, M> + Clone + 'static,
    O: 'static,
{
    template(move |ctx| {
        let entity = ctx.entity.id();
        let system = system.clone();
        Ok(ValueExpr(ctx.entity.world_scope(move |world| {
            world.register_system((move || entity).pipe(system))
        })))
    })
}

// --------------------------------------------------
// Core component type markers
// --------------------------------------------------

#[derive(SceneComponent, Clone, Default)]
pub struct WindowItem;

impl WindowItem {
    fn scene() -> impl Scene {
        bsn! {
            Properties [
                #Width Value::<LogicalLength>,
                #Height Value::<LogicalLength>,
                #AlwaysOnTop Value::<bool>,
                #FullScreen Value::<bool>,
                #Minimized Value::<bool>,
                #Maximized Value::<bool>,
                #Background Value::<Brush>,
                #Color Binding(#Background),
                #DefaultFontFamily Value::<SharedString>,
                #DefaultFontSize Value::<LogicalLength>,
                #DefaultFontWeight Value::<i32>,
                // `Image` isn't `Send` + `Sync`
                // #Icon Value::<Image>,
                #NoFrame Value::<bool>,
                #ResizeBorderWidth Value::<LogicalLength>,
                #Title Value::<SharedString>("Slint Window"),
                #SafeAreaInsets Value::<LogicalEdges>,
                #VirtualKeyboardPosition Value::<Point>,
                #VirtualKeyboardSize Value::<Size>,
            ]
        }
    }
}

#[derive(SceneComponent, Clone, Default)]
pub struct Window;

impl Window {
    fn scene() -> impl Scene {
        bsn! {
            @WindowItem
            Children [
                @MenuBar
            ]
        }
    }
}

#[derive(SceneComponent, Clone, Default)]
pub struct MenuBar;

impl MenuBar {
    fn scene() -> impl Scene {
        bsn! {
            Properties [
                #Visible Value::<bool>(true),
                Children [
                    @Menu
                ]
            ]
        }
    }
}

#[derive(SceneComponent, Clone, Default)]
pub struct Menu;

impl Menu {
    fn scene() -> impl Scene {
        bsn! {
            Properties [
                #Title Value::<SharedString>,
                #Enabled Value::<bool>,
                // `Image` isn't `Send` + `Sync`
                // #Icon Value::<Image>,
            ]
        }
    }
}

#[derive(SceneComponent, Clone, Default)]
pub struct PopupWindow;

impl PopupWindow {
    pub fn scene() -> impl Scene {}
}

#[derive(Component, Clone, Copy, Default)]
pub struct ParamName<const HASH: u128>;

pub const fn quickhash(value: &str) -> u128 {
    let hasher = sha2_const_stable::Sha512::new();

    let hasher = hasher.update(value.as_bytes());

    let bytes = hasher.finalize();

    let u128s: [u128; 4] = unsafe { mem::transmute(bytes) };

    u128s[0] ^ u128s[1] ^ u128s[2] ^ u128s[3]
}

macro_rules! param_name {
    ($name:tt) => {{
        type PName = ParamName!($name);

        fn scene() -> impl Scene {
            let name = stringify!($name).to_string();

            bsn! {
                Name(name)
                PName
            }
        }

        scene()
    }};
}

macro_rules! ParamName {
    ($name:tt) => {
        ParamName::<{ quickhash(stringify!($name)) }>
    };
}

#[derive(Component, Clone, Default)]
pub struct Dialog;

#[derive(Component, Clone, Default)]
pub struct Text;

// --------------------------------------------------
// Generated code
// --------------------------------------------------

#[derive(Clone, Default)]
pub struct TodoItem {
    pub title: SharedString,
    pub checked: bool,
}

#[derive(EntityEvent)]
pub struct TodoAdded {
    pub entity: Entity,
    pub args: (SharedString,),
}

#[derive(EntityEvent)]
pub struct RemoveDone {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct PopupConfirmed {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct ShowConfirmPopup {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct Show {
    pub entity: Entity,
}

#[derive(Event)]
pub struct ApplySortingAndFiltering;

pub fn main_window() -> impl Scene {
    let todo_model = param_name!(todo_model);
    let show_header = param_name!(show_header);
    let is_sort_by_name = param_name!(is_sort_by_name);
    let hide_done_items = param_name!(hide_done_items);
    let x = param_name!(x);
    let y = param_name!(y);
    let width = param_name!(width);

    bsn! {
        @Window
        Properties [
            todo_model ModelElements [
                Value::<TodoItem>(TodoItem { title: "Implement the .slint file", checked: true }),
                Value::<TodoItem>(TodoItem { title: "Do the Rust part", checked: false }),
                Value::<TodoItem>(TodoItem { title: "Make the C++ code", checked: false }),
                Value::<TodoItem>(TodoItem { title: "Write some JavaScript code", checked: false }),
                Value::<TodoItem>(TodoItem { title: "Test the application", checked: false }),
                Value::<TodoItem>(TodoItem { title: "Ship to customer", checked: false }),
                Value::<TodoItem>(TodoItem { title: "???", checked: false }),
                Value::<TodoItem>(TodoItem { title: "Profit", checked: false }),
            ],
            show_header Value::<bool>(false),
            is_sort_by_name Value::<bool>(false),
            hide_done_items Value::<bool>(false),
        ]
        Children [
            #ConfirmPopup
            @PopupWindow
            // TODO: Root
            Properties [
                x Value::<LogicalLength>(LogicalLength::new(40.)),
                y Value::<LogicalLength>(LogicalLength::new(100.)),
                width value_expr(|
                    In(this): In<Entity>,
                    this_query: Query<(&Root, &Children)>,
                    confirm_popup_layout: Query<&Properties, With<ParamName!(confirm_popup_layout)>>,
                    preferred_width: Query<&Value<LogicalLength>, With<ParamName!(preferred_width)>>,
                    width: Query<&Value<LogicalLength>, With<ParamName!(width)>>,
                | {
                    // TODO
                    let _ = (this, this_query, confirm_popup_layout, preferred_width, width);

                    LogicalLength::default()
                    // min(confirm_popup_layout.preferred-width, root.width - 80px);
                }),
            ]
        ]
        on(|
            event: On<ShowConfirmPopup>,
            mut commands: Commands,
            children: Query<&Children>,
            query: Query<(Entity, &Name)>,
        | {
            todo!()
        })
    }
}
