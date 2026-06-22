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

#[derive(Component, FromTemplate)]
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

/// Separate from `Name` so we can still use `#` syntax
/// to do referencing in `bsn!`.
#[derive(Component, Clone, Copy, Default)]
pub struct ParamName(pub &'static str);

#[derive(Component, Clone, Copy, Default)]
pub struct ParamId<const HASH: u128>;

pub const fn quickhash(value: &str) -> u128 {
    let hasher = sha2_const_stable::Sha512::new();

    let hasher = hasher.update(value.as_bytes());

    let bytes = hasher.finalize();

    let u128s: [u128; 4] = unsafe { mem::transmute(bytes) };

    u128s[0] ^ u128s[1] ^ u128s[2] ^ u128s[3]
}

macro_rules! param_name {
    ($name:tt) => {{
        type PName = ParamId!($name);

        fn scene() -> impl Scene {
            let name = stringify!($name);

            bsn! {
                ParamName(name)
                PName
            }
        }

        scene()
    }};
}

macro_rules! ParamId {
    ($name:tt) => {
        ParamId::<{ quickhash(stringify!($name)) }>
    };
}

// --------------------------------------------------
// Core component type markers
// --------------------------------------------------

#[derive(SceneComponent, Clone, Default)]
pub struct WindowItem;

impl WindowItem {
    fn scene() -> impl Scene {
        let width = param_name!(width);
        let height = param_name!(height);
        let always_on_top = param_name!(always_on_top);
        let full_screen = param_name!(full_screen);
        let minimized = param_name!(minimized);
        let maximized = param_name!(maximized);
        let background = param_name!(background);
        let color = param_name!(color);
        let default_font_family = param_name!(default_font_family);
        let default_font_size = param_name!(default_font_size);
        let default_font_weight = param_name!(default_font_weight);
        // `Image` isn't `Send` + `Sync`
        // #Icon Value::<Image>,
        let no_frame = param_name!(no_frame);
        let resize_border_width = param_name!(resize_border_width);
        let title = param_name!(title);
        let safe_area_insets = param_name!(safe_area_insets);
        let virtual_keyboard_position = param_name!(virtual_keyboard_position);
        let virtual_keyboard_size = param_name!(virtual_keyboard_size);

        bsn! {
            Properties [
                width Value::<LogicalLength>,
                height Value::<LogicalLength>,
                always_on_top Value::<bool>,
                full_screen Value::<bool>,
                minimized Value::<bool>,
                maximized Value::<bool>,
                background Value::<Brush>,
                color /* TODO: binding to `background` */,
                default_font_family Value::<SharedString>,
                default_font_size Value::<LogicalLength>,
                default_font_weight Value::<i32>,
                // `Image` isn't `Send` + `Sync`
                // #Icon Value::<Image>,
                no_frame Value::<bool>,
                resize_border_width Value::<LogicalLength>,
                title Value::<SharedString>("Slint Window"),
                safe_area_insets Value::<LogicalEdges>,
                virtual_keyboard_position Value::<Point>,
                virtual_keyboard_size Value::<Size>,
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
        let visible = param_name!(visible);

        bsn! {
            Properties [
                visible Value::<bool>(true),
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
        let title = param_name!(title);
        let enabled = param_name!(enabled);

        bsn! {
            Properties [
                title Value::<SharedString>,
                enabled Value::<bool>,
                // `Image` isn't `Send` + `Sync`
                // #Icon Value::<Image>,
            ]
        }
    }
}

#[derive(SceneComponent, Clone, Default)]
pub struct PopupWindow;

impl PopupWindow {
    pub fn scene() -> impl Scene {
        let width = param_name!(width);
        let height = param_name!(height);
        let close_on_click = param_name!(close_on_click);
        // let close_policy = param_name!(close_policy);
        let is_open = param_name!(is_open);

        bsn! {
            Properties [
                width Value::<LogicalLength>,
                height Value::<LogicalLength>,
                close_on_click Value::<bool>,
                // close_policy,
                is_open Value::<bool>,
            ]
        }
    }
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

    let confirm_popup = param_name!(confirm_popup);

    bsn! {
        #MainWindow
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
            confirm_popup
            @PopupWindow
            Root(#MainWindow)
            Properties [
                x Value::<LogicalLength>(LogicalLength::new(40.)),
                y Value::<LogicalLength>(LogicalLength::new(100.)),
                width value_expr(|
                    In(this): In<Entity>,
                    this_query: Query<(&Root, &Children)>,
                    confirm_popup_layout: Query<&Properties, With<ParamId!(confirm_popup_layout)>>,
                    preferred_width: Query<&Value<LogicalLength>, With<ParamId!(preferred_width)>>,
                    width: Query<&Value<LogicalLength>, With<ParamId!(width)>>,
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
