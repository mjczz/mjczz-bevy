pub mod core_2d {
    mod camera_2d {
        #![expect(deprecated)]
        use crate::{core_2d::graph::Core2d, tonemapping::{DebandDither, Tonemapping}};
        use bevy_ecs::prelude::*;
        use bevy_reflect::{std_traits::ReflectDefault, Reflect};
        use bevy_render::sync_world::SyncToRenderWorld;
        use bevy_render::{
            camera::{
                Camera, CameraMainTextureUsages, CameraProjection, CameraRenderGraph,
                OrthographicProjection,
            },
            extract_component::ExtractComponent, prelude::Msaa, primitives::Frustum,
            view::VisibleEntities,
        };
        use bevy_transform::prelude::{GlobalTransform, Transform};
        /// A 2D camera component. Enables the 2D render graph for a [`Camera`].
        #[extract_component_filter(With<Camera>)]
        #[reflect(Component, Default)]
        /**

# Required Components
[`Camera`], [`DebandDither`], [`CameraRenderGraph`], [`OrthographicProjection`], [`Frustum`], [`Tonemapping`]

 A component's [required components](bevy_ecs::component::Component#required-components) are inserted whenever it is inserted. Note that this will also insert the required components _of_ the required components, recursively, in depth-first order.*/
        pub struct Camera2d;
        impl bevy_ecs::component::Component for Camera2d
        where
            Self: Send + Sync + 'static,
        {
            const STORAGE_TYPE: bevy_ecs::component::StorageType = bevy_ecs::component::StorageType::Table;
            type Mutability = bevy_ecs::component::Mutable;
            fn register_required_components(
                requiree: bevy_ecs::component::ComponentId,
                components: &mut bevy_ecs::component::Components,
                storages: &mut bevy_ecs::storage::Storages,
                required_components: &mut bevy_ecs::component::RequiredComponents,
                inheritance_depth: u16,
                recursion_check_stack: &mut Vec<bevy_ecs::component::ComponentId>,
            ) {
                bevy_ecs::component::enforce_no_required_components_recursion(
                    components,
                    recursion_check_stack,
                );
                let self_id = components.register_component::<Self>(storages);
                recursion_check_stack.push(self_id);
                components
                    .register_required_components_manual::<
                        Self,
                        Camera,
                    >(
                        storages,
                        required_components,
                        <Camera as Default>::default,
                        inheritance_depth,
                        recursion_check_stack,
                    );
                components
                    .register_required_components_manual::<
                        Self,
                        DebandDither,
                    >(
                        storages,
                        required_components,
                        <DebandDither as Default>::default,
                        inheritance_depth,
                        recursion_check_stack,
                    );
                components
                    .register_required_components_manual::<
                        Self,
                        CameraRenderGraph,
                    >(
                        storages,
                        required_components,
                        || {
                            let x: CameraRenderGraph = (|| CameraRenderGraph::new(
                                Core2d,
                            ))()
                                .into();
                            x
                        },
                        inheritance_depth,
                        recursion_check_stack,
                    );
                components
                    .register_required_components_manual::<
                        Self,
                        OrthographicProjection,
                    >(
                        storages,
                        required_components,
                        || {
                            let x: OrthographicProjection = OrthographicProjection::default_2d()
                                .into();
                            x
                        },
                        inheritance_depth,
                        recursion_check_stack,
                    );
                components
                    .register_required_components_manual::<
                        Self,
                        Frustum,
                    >(
                        storages,
                        required_components,
                        || {
                            let x: Frustum = (|| {
                                OrthographicProjection::default_2d()
                                    .compute_frustum(
                                        &GlobalTransform::from(Transform::default()),
                                    )
                            })()
                                .into();
                            x
                        },
                        inheritance_depth,
                        recursion_check_stack,
                    );
                components
                    .register_required_components_manual::<
                        Self,
                        Tonemapping,
                    >(
                        storages,
                        required_components,
                        || {
                            let x: Tonemapping = (|| Tonemapping::None)().into();
                            x
                        },
                        inheritance_depth,
                        recursion_check_stack,
                    );
                <Camera as bevy_ecs::component::Component>::register_required_components(
                    requiree,
                    components,
                    storages,
                    required_components,
                    inheritance_depth + 1,
                    recursion_check_stack,
                );
                <DebandDither as bevy_ecs::component::Component>::register_required_components(
                    requiree,
                    components,
                    storages,
                    required_components,
                    inheritance_depth + 1,
                    recursion_check_stack,
                );
                <CameraRenderGraph as bevy_ecs::component::Component>::register_required_components(
                    requiree,
                    components,
                    storages,
                    required_components,
                    inheritance_depth + 1,
                    recursion_check_stack,
                );
                <OrthographicProjection as bevy_ecs::component::Component>::register_required_components(
                    requiree,
                    components,
                    storages,
                    required_components,
                    inheritance_depth + 1,
                    recursion_check_stack,
                );
                <Frustum as bevy_ecs::component::Component>::register_required_components(
                    requiree,
                    components,
                    storages,
                    required_components,
                    inheritance_depth + 1,
                    recursion_check_stack,
                );
                <Tonemapping as bevy_ecs::component::Component>::register_required_components(
                    requiree,
                    components,
                    storages,
                    required_components,
                    inheritance_depth + 1,
                    recursion_check_stack,
                );
                recursion_check_stack.pop();
            }
            #[allow(unused_variables)]
            fn register_component_hooks(
                hooks: &mut bevy_ecs::component::ComponentHooks,
            ) {}
            fn get_component_clone_handler() -> bevy_ecs::component::ComponentCloneHandler {
                use bevy_ecs::component::{ComponentCloneViaClone, ComponentCloneBase};
                (&&&bevy_ecs::component::ComponentCloneSpecializationWrapper::<
                    Self,
                >::default())
                    .get_component_clone_handler()
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Camera2d {
            #[inline]
            fn default() -> Camera2d {
                Camera2d {}
            }
        }
        const _: () = {
            #[allow(unused_mut)]
            impl bevy_reflect::GetTypeRegistration for Camera2d
            where
                Camera2d: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
            {
                fn get_type_registration() -> bevy_reflect::TypeRegistration {
                    let mut registration = bevy_reflect::TypeRegistration::of::<Self>();
                    registration
                        .insert::<
                            bevy_reflect::ReflectFromPtr,
                        >(bevy_reflect::FromType::<Self>::from_type());
                    registration
                        .insert::<
                            bevy_reflect::ReflectFromReflect,
                        >(bevy_reflect::FromType::<Self>::from_type());
                    registration
                        .insert::<
                            ReflectComponent,
                        >(bevy_reflect::FromType::<Self>::from_type());
                    registration
                        .insert::<
                            ReflectDefault,
                        >(bevy_reflect::FromType::<Self>::from_type());
                    registration
                }
                #[inline(never)]
                fn register_type_dependencies(
                    registry: &mut bevy_reflect::TypeRegistry,
                ) {}
            }
            impl bevy_reflect::Typed for Camera2d
            where
                Camera2d: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
            {
                #[inline]
                fn type_info() -> &'static bevy_reflect::TypeInfo {
                    static CELL: bevy_reflect::utility::NonGenericTypeInfoCell = bevy_reflect::utility::NonGenericTypeInfoCell::new();
                    CELL.get_or_set(|| {
                        bevy_reflect::TypeInfo::Struct(
                            bevy_reflect::StructInfo::new::<Self>(&[])
                                .with_custom_attributes(
                                    bevy_reflect::attributes::CustomAttributes::default(),
                                ),
                        )
                    })
                }
            }
            const _: () = {
                extern crate alloc;
                use alloc::string::ToString;
                impl bevy_reflect::TypePath for Camera2d
                where
                    Camera2d: ::core::any::Any + ::core::marker::Send
                        + ::core::marker::Sync,
                {
                    fn type_path() -> &'static str {
                        "bevy_core_pipeline::core_2d::camera_2d::Camera2d"
                    }
                    fn short_type_path() -> &'static str {
                        "Camera2d"
                    }
                    fn type_ident() -> Option<&'static str> {
                        ::core::option::Option::Some("Camera2d")
                    }
                    fn crate_name() -> Option<&'static str> {
                        ::core::option::Option::Some(
                            "bevy_core_pipeline::core_2d::camera_2d"
                                .split(':')
                                .next()
                                .unwrap(),
                        )
                    }
                    fn module_path() -> Option<&'static str> {
                        ::core::option::Option::Some(
                            "bevy_core_pipeline::core_2d::camera_2d",
                        )
                    }
                }
            };
            impl bevy_reflect::Reflect for Camera2d
            where
                Camera2d: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
            {
                #[inline]
                fn into_any(
                    self: bevy_reflect::__macro_exports::alloc_utils::Box<Self>,
                ) -> bevy_reflect::__macro_exports::alloc_utils::Box<
                    dyn ::core::any::Any,
                > {
                    self
                }
                #[inline]
                fn as_any(&self) -> &dyn ::core::any::Any {
                    self
                }
                #[inline]
                fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                    self
                }
                #[inline]
                fn into_reflect(
                    self: bevy_reflect::__macro_exports::alloc_utils::Box<Self>,
                ) -> bevy_reflect::__macro_exports::alloc_utils::Box<
                    dyn bevy_reflect::Reflect,
                > {
                    self
                }
                #[inline]
                fn as_reflect(&self) -> &dyn bevy_reflect::Reflect {
                    self
                }
                #[inline]
                fn as_reflect_mut(&mut self) -> &mut dyn bevy_reflect::Reflect {
                    self
                }
                #[inline]
                fn set(
                    &mut self,
                    value: bevy_reflect::__macro_exports::alloc_utils::Box<
                        dyn bevy_reflect::Reflect,
                    >,
                ) -> ::core::result::Result<
                    (),
                    bevy_reflect::__macro_exports::alloc_utils::Box<
                        dyn bevy_reflect::Reflect,
                    >,
                > {
                    *self = <dyn bevy_reflect::Reflect>::take(value)?;
                    ::core::result::Result::Ok(())
                }
            }
            impl bevy_reflect::Struct for Camera2d
            where
                Camera2d: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
            {
                fn field(
                    &self,
                    name: &str,
                ) -> ::core::option::Option<&dyn bevy_reflect::PartialReflect> {
                    match name {
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_mut(
                    &mut self,
                    name: &str,
                ) -> ::core::option::Option<&mut dyn bevy_reflect::PartialReflect> {
                    match name {
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_at(
                    &self,
                    index: usize,
                ) -> ::core::option::Option<&dyn bevy_reflect::PartialReflect> {
                    match index {
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_at_mut(
                    &mut self,
                    index: usize,
                ) -> ::core::option::Option<&mut dyn bevy_reflect::PartialReflect> {
                    match index {
                        _ => ::core::option::Option::None,
                    }
                }
                fn name_at(&self, index: usize) -> ::core::option::Option<&str> {
                    match index {
                        _ => ::core::option::Option::None,
                    }
                }
                fn field_len(&self) -> usize {
                    0usize
                }
                fn iter_fields(&self) -> bevy_reflect::FieldIter {
                    bevy_reflect::FieldIter::new(self)
                }
                fn clone_dynamic(&self) -> bevy_reflect::DynamicStruct {
                    let mut dynamic: bevy_reflect::DynamicStruct = ::core::default::Default::default();
                    dynamic
                        .set_represented_type(
                            bevy_reflect::PartialReflect::get_represented_type_info(self),
                        );
                    dynamic
                }
            }
            impl bevy_reflect::PartialReflect for Camera2d
            where
                Camera2d: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
            {
                #[inline]
                fn get_represented_type_info(
                    &self,
                ) -> ::core::option::Option<&'static bevy_reflect::TypeInfo> {
                    ::core::option::Option::Some(
                        <Self as bevy_reflect::Typed>::type_info(),
                    )
                }
                #[inline]
                fn clone_value(
                    &self,
                ) -> bevy_reflect::__macro_exports::alloc_utils::Box<
                    dyn bevy_reflect::PartialReflect,
                > {
                    bevy_reflect::__macro_exports::alloc_utils::Box::new(
                        bevy_reflect::Struct::clone_dynamic(self),
                    )
                }
                #[inline]
                fn try_apply(
                    &mut self,
                    value: &dyn bevy_reflect::PartialReflect,
                ) -> ::core::result::Result<(), bevy_reflect::ApplyError> {
                    if let bevy_reflect::ReflectRef::Struct(struct_value) = bevy_reflect::PartialReflect::reflect_ref(
                        value,
                    ) {
                        for (i, value) in ::core::iter::Iterator::enumerate(
                            bevy_reflect::Struct::iter_fields(struct_value),
                        ) {
                            let name = bevy_reflect::Struct::name_at(struct_value, i)
                                .unwrap();
                            if let ::core::option::Option::Some(v) = bevy_reflect::Struct::field_mut(
                                self,
                                name,
                            ) {
                                bevy_reflect::PartialReflect::try_apply(v, value)?;
                            }
                        }
                    } else {
                        return ::core::result::Result::Err(bevy_reflect::ApplyError::MismatchedKinds {
                            from_kind: bevy_reflect::PartialReflect::reflect_kind(value),
                            to_kind: bevy_reflect::ReflectKind::Struct,
                        });
                    }
                    ::core::result::Result::Ok(())
                }
                #[inline]
                fn reflect_kind(&self) -> bevy_reflect::ReflectKind {
                    bevy_reflect::ReflectKind::Struct
                }
                #[inline]
                fn reflect_ref(&self) -> bevy_reflect::ReflectRef {
                    bevy_reflect::ReflectRef::Struct(self)
                }
                #[inline]
                fn reflect_mut(&mut self) -> bevy_reflect::ReflectMut {
                    bevy_reflect::ReflectMut::Struct(self)
                }
                #[inline]
                fn reflect_owned(
                    self: bevy_reflect::__macro_exports::alloc_utils::Box<Self>,
                ) -> bevy_reflect::ReflectOwned {
                    bevy_reflect::ReflectOwned::Struct(self)
                }
                #[inline]
                fn try_into_reflect(
                    self: bevy_reflect::__macro_exports::alloc_utils::Box<Self>,
                ) -> ::core::result::Result<
                    bevy_reflect::__macro_exports::alloc_utils::Box<
                        dyn bevy_reflect::Reflect,
                    >,
                    bevy_reflect::__macro_exports::alloc_utils::Box<
                        dyn bevy_reflect::PartialReflect,
                    >,
                > {
                    ::core::result::Result::Ok(self)
                }
                #[inline]
                fn try_as_reflect(
                    &self,
                ) -> ::core::option::Option<&dyn bevy_reflect::Reflect> {
                    ::core::option::Option::Some(self)
                }
                #[inline]
                fn try_as_reflect_mut(
                    &mut self,
                ) -> ::core::option::Option<&mut dyn bevy_reflect::Reflect> {
                    ::core::option::Option::Some(self)
                }
                #[inline]
                fn into_partial_reflect(
                    self: bevy_reflect::__macro_exports::alloc_utils::Box<Self>,
                ) -> bevy_reflect::__macro_exports::alloc_utils::Box<
                    dyn bevy_reflect::PartialReflect,
                > {
                    self
                }
                #[inline]
                fn as_partial_reflect(&self) -> &dyn bevy_reflect::PartialReflect {
                    self
                }
                #[inline]
                fn as_partial_reflect_mut(
                    &mut self,
                ) -> &mut dyn bevy_reflect::PartialReflect {
                    self
                }
                fn reflect_partial_eq(
                    &self,
                    value: &dyn bevy_reflect::PartialReflect,
                ) -> ::core::option::Option<bool> {
                    (bevy_reflect::struct_partial_eq)(self, value)
                }
            }
            impl bevy_reflect::FromReflect for Camera2d
            where
                Camera2d: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
            {
                fn from_reflect(
                    reflect: &dyn bevy_reflect::PartialReflect,
                ) -> ::core::option::Option<Self> {
                    if let bevy_reflect::ReflectRef::Struct(__ref_struct) = bevy_reflect::PartialReflect::reflect_ref(
                        reflect,
                    ) {
                        let mut __this = <Self as ::core::default::Default>::default();
                        ::core::option::Option::Some(__this)
                    } else {
                        ::core::option::Option::None
                    }
                }
            }
        };
        #[automatically_derived]
        impl ::core::clone::Clone for Camera2d {
            #[inline]
            fn clone(&self) -> Camera2d {
                Camera2d
            }
        }
        impl bevy_render::extract_component::ExtractComponent for Camera2d
        where
            Self: Clone,
        {
            type QueryData = &'static Self;
            type QueryFilter = With<Camera>;
            type Out = Self;
            fn extract_component(
                item: bevy_ecs::query::QueryItem<'_, Self::QueryData>,
            ) -> Option<Self::Out> {
                Some(item.clone())
            }
        }
        #[deprecated(
            since = "0.15.0",
            note = "Use the `Camera2d` component instead. Inserting it will now also insert the other components required by it automatically."
        )]
        pub struct Camera2dBundle {
            pub camera: Camera,
            pub camera_render_graph: CameraRenderGraph,
            pub projection: OrthographicProjection,
            pub visible_entities: VisibleEntities,
            pub frustum: Frustum,
            pub transform: Transform,
            pub global_transform: GlobalTransform,
            pub camera_2d: Camera2d,
            pub tonemapping: Tonemapping,
            pub deband_dither: DebandDither,
            pub main_texture_usages: CameraMainTextureUsages,
            pub msaa: Msaa,
            /// Marker component that indicates that its entity needs to be synchronized to the render world
            pub sync: SyncToRenderWorld,
        }
        unsafe impl bevy_ecs::bundle::Bundle for Camera2dBundle {
            fn component_ids(
                components: &mut bevy_ecs::component::Components,
                storages: &mut bevy_ecs::storage::Storages,
                ids: &mut impl FnMut(bevy_ecs::component::ComponentId),
            ) {
                <Camera as bevy_ecs::bundle::Bundle>::component_ids(
                    components,
                    storages,
                    &mut *ids,
                );
                <CameraRenderGraph as bevy_ecs::bundle::Bundle>::component_ids(
                    components,
                    storages,
                    &mut *ids,
                );
                <OrthographicProjection as bevy_ecs::bundle::Bundle>::component_ids(
                    components,
                    storages,
                    &mut *ids,
                );
                <VisibleEntities as bevy_ecs::bundle::Bundle>::component_ids(
                    components,
                    storages,
                    &mut *ids,
                );
                <Frustum as bevy_ecs::bundle::Bundle>::component_ids(
                    components,
                    storages,
                    &mut *ids,
                );
                <Transform as bevy_ecs::bundle::Bundle>::component_ids(
                    components,
                    storages,
                    &mut *ids,
                );
                <GlobalTransform as bevy_ecs::bundle::Bundle>::component_ids(
                    components,
                    storages,
                    &mut *ids,
                );
                <Camera2d as bevy_ecs::bundle::Bundle>::component_ids(
                    components,
                    storages,
                    &mut *ids,
                );
                <Tonemapping as bevy_ecs::bundle::Bundle>::component_ids(
                    components,
                    storages,
                    &mut *ids,
                );
                <DebandDither as bevy_ecs::bundle::Bundle>::component_ids(
                    components,
                    storages,
                    &mut *ids,
                );
                <CameraMainTextureUsages as bevy_ecs::bundle::Bundle>::component_ids(
                    components,
                    storages,
                    &mut *ids,
                );
                <Msaa as bevy_ecs::bundle::Bundle>::component_ids(
                    components,
                    storages,
                    &mut *ids,
                );
                <SyncToRenderWorld as bevy_ecs::bundle::Bundle>::component_ids(
                    components,
                    storages,
                    &mut *ids,
                );
            }
            fn get_component_ids(
                components: &bevy_ecs::component::Components,
                ids: &mut impl FnMut(Option<bevy_ecs::component::ComponentId>),
            ) {
                <Camera as bevy_ecs::bundle::Bundle>::get_component_ids(
                    components,
                    &mut *ids,
                );
                <CameraRenderGraph as bevy_ecs::bundle::Bundle>::get_component_ids(
                    components,
                    &mut *ids,
                );
                <OrthographicProjection as bevy_ecs::bundle::Bundle>::get_component_ids(
                    components,
                    &mut *ids,
                );
                <VisibleEntities as bevy_ecs::bundle::Bundle>::get_component_ids(
                    components,
                    &mut *ids,
                );
                <Frustum as bevy_ecs::bundle::Bundle>::get_component_ids(
                    components,
                    &mut *ids,
                );
                <Transform as bevy_ecs::bundle::Bundle>::get_component_ids(
                    components,
                    &mut *ids,
                );
                <GlobalTransform as bevy_ecs::bundle::Bundle>::get_component_ids(
                    components,
                    &mut *ids,
                );
                <Camera2d as bevy_ecs::bundle::Bundle>::get_component_ids(
                    components,
                    &mut *ids,
                );
                <Tonemapping as bevy_ecs::bundle::Bundle>::get_component_ids(
                    components,
                    &mut *ids,
                );
                <DebandDither as bevy_ecs::bundle::Bundle>::get_component_ids(
                    components,
                    &mut *ids,
                );
                <CameraMainTextureUsages as bevy_ecs::bundle::Bundle>::get_component_ids(
                    components,
                    &mut *ids,
                );
                <Msaa as bevy_ecs::bundle::Bundle>::get_component_ids(
                    components,
                    &mut *ids,
                );
                <SyncToRenderWorld as bevy_ecs::bundle::Bundle>::get_component_ids(
                    components,
                    &mut *ids,
                );
            }
            #[allow(unused_variables, non_snake_case)]
            unsafe fn from_components<__T, __F>(ctx: &mut __T, func: &mut __F) -> Self
            where
                __F: FnMut(&mut __T) -> bevy_ecs::ptr::OwningPtr<'_>,
            {
                Self {
                    camera: <Camera as bevy_ecs::bundle::Bundle>::from_components(
                        ctx,
                        &mut *func,
                    ),
                    camera_render_graph: <CameraRenderGraph as bevy_ecs::bundle::Bundle>::from_components(
                        ctx,
                        &mut *func,
                    ),
                    projection: <OrthographicProjection as bevy_ecs::bundle::Bundle>::from_components(
                        ctx,
                        &mut *func,
                    ),
                    visible_entities: <VisibleEntities as bevy_ecs::bundle::Bundle>::from_components(
                        ctx,
                        &mut *func,
                    ),
                    frustum: <Frustum as bevy_ecs::bundle::Bundle>::from_components(
                        ctx,
                        &mut *func,
                    ),
                    transform: <Transform as bevy_ecs::bundle::Bundle>::from_components(
                        ctx,
                        &mut *func,
                    ),
                    global_transform: <GlobalTransform as bevy_ecs::bundle::Bundle>::from_components(
                        ctx,
                        &mut *func,
                    ),
                    camera_2d: <Camera2d as bevy_ecs::bundle::Bundle>::from_components(
                        ctx,
                        &mut *func,
                    ),
                    tonemapping: <Tonemapping as bevy_ecs::bundle::Bundle>::from_components(
                        ctx,
                        &mut *func,
                    ),
                    deband_dither: <DebandDither as bevy_ecs::bundle::Bundle>::from_components(
                        ctx,
                        &mut *func,
                    ),
                    main_texture_usages: <CameraMainTextureUsages as bevy_ecs::bundle::Bundle>::from_components(
                        ctx,
                        &mut *func,
                    ),
                    msaa: <Msaa as bevy_ecs::bundle::Bundle>::from_components(
                        ctx,
                        &mut *func,
                    ),
                    sync: <SyncToRenderWorld as bevy_ecs::bundle::Bundle>::from_components(
                        ctx,
                        &mut *func,
                    ),
                }
            }
            fn register_required_components(
                components: &mut bevy_ecs::component::Components,
                storages: &mut bevy_ecs::storage::Storages,
                required_components: &mut bevy_ecs::component::RequiredComponents,
            ) {
                <Camera as bevy_ecs::bundle::Bundle>::register_required_components(
                    components,
                    storages,
                    required_components,
                );
                <CameraRenderGraph as bevy_ecs::bundle::Bundle>::register_required_components(
                    components,
                    storages,
                    required_components,
                );
                <OrthographicProjection as bevy_ecs::bundle::Bundle>::register_required_components(
                    components,
                    storages,
                    required_components,
                );
                <VisibleEntities as bevy_ecs::bundle::Bundle>::register_required_components(
                    components,
                    storages,
                    required_components,
                );
                <Frustum as bevy_ecs::bundle::Bundle>::register_required_components(
                    components,
                    storages,
                    required_components,
                );
                <Transform as bevy_ecs::bundle::Bundle>::register_required_components(
                    components,
                    storages,
                    required_components,
                );
                <GlobalTransform as bevy_ecs::bundle::Bundle>::register_required_components(
                    components,
                    storages,
                    required_components,
                );
                <Camera2d as bevy_ecs::bundle::Bundle>::register_required_components(
                    components,
                    storages,
                    required_components,
                );
                <Tonemapping as bevy_ecs::bundle::Bundle>::register_required_components(
                    components,
                    storages,
                    required_components,
                );
                <DebandDither as bevy_ecs::bundle::Bundle>::register_required_components(
                    components,
                    storages,
                    required_components,
                );
                <CameraMainTextureUsages as bevy_ecs::bundle::Bundle>::register_required_components(
                    components,
                    storages,
                    required_components,
                );
                <Msaa as bevy_ecs::bundle::Bundle>::register_required_components(
                    components,
                    storages,
                    required_components,
                );
                <SyncToRenderWorld as bevy_ecs::bundle::Bundle>::register_required_components(
                    components,
                    storages,
                    required_components,
                );
            }
        }
        impl bevy_ecs::bundle::DynamicBundle for Camera2dBundle {
            #[allow(unused_variables)]
            #[inline]
            fn get_components(
                self,
                func: &mut impl FnMut(
                    bevy_ecs::component::StorageType,
                    bevy_ecs::ptr::OwningPtr<'_>,
                ),
            ) {
                self.camera.get_components(&mut *func);
                self.camera_render_graph.get_components(&mut *func);
                self.projection.get_components(&mut *func);
                self.visible_entities.get_components(&mut *func);
                self.frustum.get_components(&mut *func);
                self.transform.get_components(&mut *func);
                self.global_transform.get_components(&mut *func);
                self.camera_2d.get_components(&mut *func);
                self.tonemapping.get_components(&mut *func);
                self.deband_dither.get_components(&mut *func);
                self.main_texture_usages.get_components(&mut *func);
                self.msaa.get_components(&mut *func);
                self.sync.get_components(&mut *func);
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Camera2dBundle {
            #[inline]
            fn clone(&self) -> Camera2dBundle {
                Camera2dBundle {
                    camera: ::core::clone::Clone::clone(&self.camera),
                    camera_render_graph: ::core::clone::Clone::clone(
                        &self.camera_render_graph,
                    ),
                    projection: ::core::clone::Clone::clone(&self.projection),
                    visible_entities: ::core::clone::Clone::clone(
                        &self.visible_entities,
                    ),
                    frustum: ::core::clone::Clone::clone(&self.frustum),
                    transform: ::core::clone::Clone::clone(&self.transform),
                    global_transform: ::core::clone::Clone::clone(
                        &self.global_transform,
                    ),
                    camera_2d: ::core::clone::Clone::clone(&self.camera_2d),
                    tonemapping: ::core::clone::Clone::clone(&self.tonemapping),
                    deband_dither: ::core::clone::Clone::clone(&self.deband_dither),
                    main_texture_usages: ::core::clone::Clone::clone(
                        &self.main_texture_usages,
                    ),
                    msaa: ::core::clone::Clone::clone(&self.msaa),
                    sync: ::core::clone::Clone::clone(&self.sync),
                }
            }
        }
        impl Default for Camera2dBundle {
            fn default() -> Self {
                let projection = OrthographicProjection::default_2d();
                let transform = Transform::default();
                let frustum = projection
                    .compute_frustum(&GlobalTransform::from(transform));
                Self {
                    camera_render_graph: CameraRenderGraph::new(Core2d),
                    projection,
                    visible_entities: VisibleEntities::default(),
                    frustum,
                    transform,
                    global_transform: Default::default(),
                    camera: Camera::default(),
                    camera_2d: Camera2d,
                    tonemapping: Tonemapping::None,
                    deband_dither: DebandDither::Disabled,
                    main_texture_usages: Default::default(),
                    msaa: Default::default(),
                    sync: Default::default(),
                }
            }
        }
        impl Camera2dBundle {
            /// Create an orthographic projection camera with a custom `Z` position.
            ///
            /// The camera is placed at `Z=far-0.1`, looking toward the world origin `(0,0,0)`.
            /// Its orthographic projection extends from `0.0` to `-far` in camera view space,
            /// corresponding to `Z=far-0.1` (closest to camera) to `Z=-0.1` (furthest away from
            /// camera) in world space.
            pub fn new_with_far(far: f32) -> Self {
                let projection = OrthographicProjection {
                    far,
                    ..OrthographicProjection::default_2d()
                };
                let transform = Transform::from_xyz(0.0, 0.0, far - 0.1);
                let frustum = projection
                    .compute_frustum(&GlobalTransform::from(transform));
                Self {
                    camera_render_graph: CameraRenderGraph::new(Core2d),
                    projection,
                    visible_entities: VisibleEntities::default(),
                    frustum,
                    transform,
                    global_transform: Default::default(),
                    camera: Camera::default(),
                    camera_2d: Camera2d,
                    tonemapping: Tonemapping::None,
                    deband_dither: DebandDither::Disabled,
                    main_texture_usages: Default::default(),
                    msaa: Default::default(),
                    sync: Default::default(),
                }
            }
        }
    }
    mod main_opaque_pass_2d_node {
        use crate::core_2d::Opaque2d;
        use bevy_ecs::{prelude::World, query::QueryItem};
        use bevy_render::{
            camera::ExtractedCamera, diagnostic::RecordDiagnostics,
            render_graph::{NodeRunError, RenderGraphContext, ViewNode},
            render_phase::{TrackedRenderPass, ViewBinnedRenderPhases},
            render_resource::{CommandEncoderDescriptor, RenderPassDescriptor, StoreOp},
            renderer::RenderContext, view::{ViewDepthTexture, ViewTarget},
        };
        use bevy_utils::tracing::error;
        use super::AlphaMask2d;
        /// A [`bevy_render::render_graph::Node`] that runs the
        /// [`Opaque2d`] [`ViewBinnedRenderPhases`] and [`AlphaMask2d`] [`ViewBinnedRenderPhases`]
        pub struct MainOpaquePass2dNode;
        #[automatically_derived]
        impl ::core::default::Default for MainOpaquePass2dNode {
            #[inline]
            fn default() -> MainOpaquePass2dNode {
                MainOpaquePass2dNode {}
            }
        }
        impl ViewNode for MainOpaquePass2dNode {
            type ViewQuery = (
                &'static ExtractedCamera,
                &'static ViewTarget,
                &'static ViewDepthTexture,
            );
            fn run<'w>(
                &self,
                graph: &mut RenderGraphContext,
                render_context: &mut RenderContext<'w>,
                (camera, target, depth): QueryItem<'w, Self::ViewQuery>,
                world: &'w World,
            ) -> Result<(), NodeRunError> {
                let (Some(opaque_phases), Some(alpha_mask_phases)) = (
                    world.get_resource::<ViewBinnedRenderPhases<Opaque2d>>(),
                    world.get_resource::<ViewBinnedRenderPhases<AlphaMask2d>>(),
                ) else {
                    return Ok(());
                };
                let diagnostics = render_context.diagnostic_recorder();
                let color_attachments = [Some(target.get_color_attachment())];
                let depth_stencil_attachment = Some(
                    depth.get_attachment(StoreOp::Store),
                );
                let view_entity = graph.view_entity();
                let (Some(opaque_phase), Some(alpha_mask_phase)) = (
                    opaque_phases.get(&view_entity),
                    alpha_mask_phases.get(&view_entity),
                ) else {
                    return Ok(());
                };
                render_context
                    .add_command_buffer_generation_task(move |render_device| {
                        let mut command_encoder = render_device
                            .create_command_encoder(
                                &CommandEncoderDescriptor {
                                    label: Some("main_opaque_pass_2d_command_encoder"),
                                },
                            );
                        let render_pass = command_encoder
                            .begin_render_pass(
                                &RenderPassDescriptor {
                                    label: Some("main_opaque_pass_2d"),
                                    color_attachments: &color_attachments,
                                    depth_stencil_attachment,
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                },
                            );
                        let mut render_pass = TrackedRenderPass::new(
                            &render_device,
                            render_pass,
                        );
                        let pass_span = diagnostics
                            .pass_span(&mut render_pass, "main_opaque_pass_2d");
                        if let Some(viewport) = camera.viewport.as_ref() {
                            render_pass.set_camera_viewport(viewport);
                        }
                        if !opaque_phase.is_empty() {
                            if let Err(err) = opaque_phase
                                .render(&mut render_pass, world, view_entity)
                            {
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event crates/bevy_core_pipeline/src/core_2d/main_opaque_pass_2d_node.rs:85",
                                                "bevy_core_pipeline::core_2d::main_opaque_pass_2d_node",
                                                ::tracing::Level::ERROR,
                                                ::core::option::Option::Some(
                                                    "crates/bevy_core_pipeline/src/core_2d/main_opaque_pass_2d_node.rs",
                                                ),
                                                ::core::option::Option::Some(85u32),
                                                ::core::option::Option::Some(
                                                    "bevy_core_pipeline::core_2d::main_opaque_pass_2d_node",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::ERROR
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &format_args!(
                                                                    "Error encountered while rendering the 2d opaque phase {0:?}",
                                                                    err,
                                                                ) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                    }
                                };
                            }
                        }
                        if !alpha_mask_phase.is_empty() {
                            if let Err(err) = alpha_mask_phase
                                .render(&mut render_pass, world, view_entity)
                            {
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event crates/bevy_core_pipeline/src/core_2d/main_opaque_pass_2d_node.rs:94",
                                                "bevy_core_pipeline::core_2d::main_opaque_pass_2d_node",
                                                ::tracing::Level::ERROR,
                                                ::core::option::Option::Some(
                                                    "crates/bevy_core_pipeline/src/core_2d/main_opaque_pass_2d_node.rs",
                                                ),
                                                ::core::option::Option::Some(94u32),
                                                ::core::option::Option::Some(
                                                    "bevy_core_pipeline::core_2d::main_opaque_pass_2d_node",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::ERROR
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::core::iter::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::core::option::Option::Some(
                                                                &format_args!(
                                                                    "Error encountered while rendering the 2d alpha mask phase {0:?}",
                                                                    err,
                                                                ) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                    }
                                };
                            }
                        }
                        pass_span.end(&mut render_pass);
                        drop(render_pass);
                        command_encoder.finish()
                    });
                Ok(())
            }
        }
    }
    mod main_transparent_pass_2d_node {
        use crate::core_2d::Transparent2d;
        use bevy_ecs::prelude::*;
        use bevy_render::{
            camera::ExtractedCamera, diagnostic::RecordDiagnostics,
            render_graph::{NodeRunError, RenderGraphContext, ViewNode},
            render_phase::ViewSortedRenderPhases,
            render_resource::{RenderPassDescriptor, StoreOp},
            renderer::RenderContext, view::{ViewDepthTexture, ViewTarget},
        };
        use bevy_utils::tracing::error;
        pub struct MainTransparentPass2dNode {}
        #[automatically_derived]
        impl ::core::default::Default for MainTransparentPass2dNode {
            #[inline]
            fn default() -> MainTransparentPass2dNode {
                MainTransparentPass2dNode {}
            }
        }
        impl ViewNode for MainTransparentPass2dNode {
            type ViewQuery = (
                &'static ExtractedCamera,
                &'static ViewTarget,
                &'static ViewDepthTexture,
            );
            fn run<'w>(
                &self,
                graph: &mut RenderGraphContext,
                render_context: &mut RenderContext<'w>,
                (camera, target, depth): bevy_ecs::query::QueryItem<'w, Self::ViewQuery>,
                world: &'w World,
            ) -> Result<(), NodeRunError> {
                let Some(transparent_phases) = world
                    .get_resource::<ViewSortedRenderPhases<Transparent2d>>() else {
                    return Ok(());
                };
                let view_entity = graph.view_entity();
                let Some(transparent_phase) = transparent_phases.get(&view_entity) else {
                    return Ok(());
                };
                {
                    let diagnostics = render_context.diagnostic_recorder();
                    let mut render_pass = render_context
                        .begin_tracked_render_pass(RenderPassDescriptor {
                            label: Some("main_transparent_pass_2d"),
                            color_attachments: &[Some(target.get_color_attachment())],
                            depth_stencil_attachment: Some(
                                depth.get_attachment(StoreOp::Store),
                            ),
                            timestamp_writes: None,
                            occlusion_query_set: None,
                        });
                    let pass_span = diagnostics
                        .pass_span(&mut render_pass, "main_transparent_pass_2d");
                    if let Some(viewport) = camera.viewport.as_ref() {
                        render_pass.set_camera_viewport(viewport);
                    }
                    if !transparent_phase.items.is_empty() {
                        if let Err(err) = transparent_phase
                            .render(&mut render_pass, world, view_entity)
                        {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event crates/bevy_core_pipeline/src/core_2d/main_transparent_pass_2d_node.rs:76",
                                            "bevy_core_pipeline::core_2d::main_transparent_pass_2d_node",
                                            ::tracing::Level::ERROR,
                                            ::core::option::Option::Some(
                                                "crates/bevy_core_pipeline/src/core_2d/main_transparent_pass_2d_node.rs",
                                            ),
                                            ::core::option::Option::Some(76u32),
                                            ::core::option::Option::Some(
                                                "bevy_core_pipeline::core_2d::main_transparent_pass_2d_node",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!(
                                                                "Error encountered while rendering the transparent 2D phase {0:?}",
                                                                err,
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                        }
                    }
                    pass_span.end(&mut render_pass);
                }
                Ok(())
            }
        }
    }
    pub mod graph {
        use bevy_render::render_graph::{RenderLabel, RenderSubGraph};
        pub struct Core2d;
        #[automatically_derived]
        impl ::core::fmt::Debug for Core2d {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "Core2d")
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Core2d {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Core2d {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Core2d {
            #[inline]
            fn eq(&self, other: &Core2d) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Core2d {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Core2d {
            #[inline]
            fn clone(&self) -> Core2d {
                Core2d
            }
        }
        const _: () = {
            extern crate alloc;
            impl bevy_render::render_graph::RenderSubGraph for Core2d
            where
                Self: 'static + Send + Sync + Clone + Eq + ::core::fmt::Debug
                    + ::core::hash::Hash,
            {
                fn dyn_clone(
                    &self,
                ) -> alloc::boxed::Box<dyn bevy_render::render_graph::RenderSubGraph> {
                    alloc::boxed::Box::new(::core::clone::Clone::clone(self))
                }
                fn as_dyn_eq(&self) -> &dyn bevy_render::render_graph::DynEq {
                    self
                }
                fn dyn_hash(&self, mut state: &mut dyn ::core::hash::Hasher) {
                    let ty_id = ::core::any::TypeId::of::<Self>();
                    ::core::hash::Hash::hash(&ty_id, &mut state);
                    ::core::hash::Hash::hash(self, &mut state);
                }
            }
        };
        pub mod input {
            pub const VIEW_ENTITY: &str = "view_entity";
        }
        pub enum Node2d {
            MsaaWriteback,
            StartMainPass,
            MainOpaquePass,
            MainTransparentPass,
            EndMainPass,
            Bloom,
            PostProcessing,
            Tonemapping,
            Fxaa,
            Smaa,
            Upscaling,
            ContrastAdaptiveSharpening,
            EndMainPassPostProcessing,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Node2d {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Node2d::MsaaWriteback => "MsaaWriteback",
                        Node2d::StartMainPass => "StartMainPass",
                        Node2d::MainOpaquePass => "MainOpaquePass",
                        Node2d::MainTransparentPass => "MainTransparentPass",
                        Node2d::EndMainPass => "EndMainPass",
                        Node2d::Bloom => "Bloom",
                        Node2d::PostProcessing => "PostProcessing",
                        Node2d::Tonemapping => "Tonemapping",
                        Node2d::Fxaa => "Fxaa",
                        Node2d::Smaa => "Smaa",
                        Node2d::Upscaling => "Upscaling",
                        Node2d::ContrastAdaptiveSharpening => {
                            "ContrastAdaptiveSharpening"
                        }
                        Node2d::EndMainPassPostProcessing => "EndMainPassPostProcessing",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Node2d {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Node2d {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Node2d {
            #[inline]
            fn eq(&self, other: &Node2d) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Node2d {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Node2d {
            #[inline]
            fn clone(&self) -> Node2d {
                match self {
                    Node2d::MsaaWriteback => Node2d::MsaaWriteback,
                    Node2d::StartMainPass => Node2d::StartMainPass,
                    Node2d::MainOpaquePass => Node2d::MainOpaquePass,
                    Node2d::MainTransparentPass => Node2d::MainTransparentPass,
                    Node2d::EndMainPass => Node2d::EndMainPass,
                    Node2d::Bloom => Node2d::Bloom,
                    Node2d::PostProcessing => Node2d::PostProcessing,
                    Node2d::Tonemapping => Node2d::Tonemapping,
                    Node2d::Fxaa => Node2d::Fxaa,
                    Node2d::Smaa => Node2d::Smaa,
                    Node2d::Upscaling => Node2d::Upscaling,
                    Node2d::ContrastAdaptiveSharpening => {
                        Node2d::ContrastAdaptiveSharpening
                    }
                    Node2d::EndMainPassPostProcessing => {
                        Node2d::EndMainPassPostProcessing
                    }
                }
            }
        }
        const _: () = {
            extern crate alloc;
            impl bevy_render::render_graph::RenderLabel for Node2d
            where
                Self: 'static + Send + Sync + Clone + Eq + ::core::fmt::Debug
                    + ::core::hash::Hash,
            {
                fn dyn_clone(
                    &self,
                ) -> alloc::boxed::Box<dyn bevy_render::render_graph::RenderLabel> {
                    alloc::boxed::Box::new(::core::clone::Clone::clone(self))
                }
                fn as_dyn_eq(&self) -> &dyn bevy_render::render_graph::DynEq {
                    self
                }
                fn dyn_hash(&self, mut state: &mut dyn ::core::hash::Hasher) {
                    let ty_id = ::core::any::TypeId::of::<Self>();
                    ::core::hash::Hash::hash(&ty_id, &mut state);
                    ::core::hash::Hash::hash(self, &mut state);
                }
            }
        };
    }
    use core::ops::Range;
    use bevy_asset::UntypedAssetId;
    use bevy_render::{
        batching::gpu_preprocessing::GpuPreprocessingMode, render_phase::PhaseItemBinKey,
    };
    use bevy_utils::HashMap;
    pub use camera_2d::*;
    pub use main_opaque_pass_2d_node::*;
    pub use main_transparent_pass_2d_node::*;
    use crate::{tonemapping::TonemappingNode, upscaling::UpscalingNode};
    use bevy_app::{App, Plugin};
    use bevy_ecs::{entity::EntityHashSet, prelude::*};
    use bevy_math::FloatOrd;
    use bevy_render::{
        camera::{Camera, ExtractedCamera},
        extract_component::ExtractComponentPlugin,
        render_graph::{EmptyNode, RenderGraphApp, ViewNodeRunner},
        render_phase::{
            sort_phase_system, BinnedPhaseItem, CachedRenderPipelinePhaseItem,
            DrawFunctionId, DrawFunctions, PhaseItem, PhaseItemExtraIndex,
            SortedPhaseItem, ViewBinnedRenderPhases, ViewSortedRenderPhases,
        },
        render_resource::{
            BindGroupId, CachedRenderPipelineId, Extent3d, TextureDescriptor,
            TextureDimension, TextureFormat, TextureUsages,
        },
        renderer::RenderDevice, sync_world::{MainEntity, RenderEntity},
        texture::TextureCache, view::{Msaa, ViewDepthTexture},
        Extract, ExtractSchedule, Render, RenderApp, RenderSet,
    };
    use self::graph::{Core2d, Node2d};
    pub const CORE_2D_DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;
    pub struct Core2dPlugin;
    impl Plugin for Core2dPlugin {
        fn build(&self, app: &mut App) {
            app.register_type::<Camera2d>()
                .add_plugins(ExtractComponentPlugin::<Camera2d>::default());
            let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
                return;
            };
            render_app
                .init_resource::<DrawFunctions<Opaque2d>>()
                .init_resource::<DrawFunctions<AlphaMask2d>>()
                .init_resource::<DrawFunctions<Transparent2d>>()
                .init_resource::<ViewSortedRenderPhases<Transparent2d>>()
                .init_resource::<ViewBinnedRenderPhases<Opaque2d>>()
                .init_resource::<ViewBinnedRenderPhases<AlphaMask2d>>()
                .add_systems(ExtractSchedule, extract_core_2d_camera_phases)
                .add_systems(
                    Render,
                    (
                        sort_phase_system::<Transparent2d>.in_set(RenderSet::PhaseSort),
                        prepare_core_2d_depth_textures
                            .in_set(RenderSet::PrepareResources),
                    ),
                );
            render_app
                .add_render_sub_graph(Core2d)
                .add_render_graph_node::<EmptyNode>(Core2d, Node2d::StartMainPass)
                .add_render_graph_node::<
                    ViewNodeRunner<MainOpaquePass2dNode>,
                >(Core2d, Node2d::MainOpaquePass)
                .add_render_graph_node::<
                    ViewNodeRunner<MainTransparentPass2dNode>,
                >(Core2d, Node2d::MainTransparentPass)
                .add_render_graph_node::<EmptyNode>(Core2d, Node2d::EndMainPass)
                .add_render_graph_node::<
                    ViewNodeRunner<TonemappingNode>,
                >(Core2d, Node2d::Tonemapping)
                .add_render_graph_node::<
                    EmptyNode,
                >(Core2d, Node2d::EndMainPassPostProcessing)
                .add_render_graph_node::<
                    ViewNodeRunner<UpscalingNode>,
                >(Core2d, Node2d::Upscaling)
                .add_render_graph_edges(
                    Core2d,
                    (
                        Node2d::StartMainPass,
                        Node2d::MainOpaquePass,
                        Node2d::MainTransparentPass,
                        Node2d::EndMainPass,
                        Node2d::Tonemapping,
                        Node2d::EndMainPassPostProcessing,
                        Node2d::Upscaling,
                    ),
                );
        }
    }
    /// Opaque 2D [`BinnedPhaseItem`]s.
    pub struct Opaque2d {
        /// The key, which determines which can be batched.
        pub key: Opaque2dBinKey,
        /// An entity from which data will be fetched, including the mesh if
        /// applicable.
        pub representative_entity: (Entity, MainEntity),
        /// The ranges of instances.
        pub batch_range: Range<u32>,
        /// An extra index, which is either a dynamic offset or an index in the
        /// indirect parameters list.
        pub extra_index: PhaseItemExtraIndex,
    }
    /// Data that must be identical in order to batch phase items together.
    pub struct Opaque2dBinKey {
        /// The identifier of the render pipeline.
        pub pipeline: CachedRenderPipelineId,
        /// The function used to draw.
        pub draw_function: DrawFunctionId,
        /// The asset that this phase item is associated with.
        ///
        /// Normally, this is the ID of the mesh, but for non-mesh items it might be
        /// the ID of another type of asset.
        pub asset_id: UntypedAssetId,
        /// The ID of a bind group specific to the material.
        pub material_bind_group_id: Option<BindGroupId>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Opaque2dBinKey {
        #[inline]
        fn clone(&self) -> Opaque2dBinKey {
            Opaque2dBinKey {
                pipeline: ::core::clone::Clone::clone(&self.pipeline),
                draw_function: ::core::clone::Clone::clone(&self.draw_function),
                asset_id: ::core::clone::Clone::clone(&self.asset_id),
                material_bind_group_id: ::core::clone::Clone::clone(
                    &self.material_bind_group_id,
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Opaque2dBinKey {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Opaque2dBinKey {
        #[inline]
        fn eq(&self, other: &Opaque2dBinKey) -> bool {
            self.pipeline == other.pipeline && self.draw_function == other.draw_function
                && self.asset_id == other.asset_id
                && self.material_bind_group_id == other.material_bind_group_id
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Opaque2dBinKey {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<CachedRenderPipelineId>;
            let _: ::core::cmp::AssertParamIsEq<DrawFunctionId>;
            let _: ::core::cmp::AssertParamIsEq<UntypedAssetId>;
            let _: ::core::cmp::AssertParamIsEq<Option<BindGroupId>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Opaque2dBinKey {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Opaque2dBinKey,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.pipeline, &other.pipeline) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(
                        &self.draw_function,
                        &other.draw_function,
                    ) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                &self.asset_id,
                                &other.asset_id,
                            ) {
                                ::core::option::Option::Some(
                                    ::core::cmp::Ordering::Equal,
                                ) => {
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &self.material_bind_group_id,
                                        &other.material_bind_group_id,
                                    )
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Opaque2dBinKey {
        #[inline]
        fn cmp(&self, other: &Opaque2dBinKey) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.pipeline, &other.pipeline) {
                ::core::cmp::Ordering::Equal => {
                    match ::core::cmp::Ord::cmp(
                        &self.draw_function,
                        &other.draw_function,
                    ) {
                        ::core::cmp::Ordering::Equal => {
                            match ::core::cmp::Ord::cmp(
                                &self.asset_id,
                                &other.asset_id,
                            ) {
                                ::core::cmp::Ordering::Equal => {
                                    ::core::cmp::Ord::cmp(
                                        &self.material_bind_group_id,
                                        &other.material_bind_group_id,
                                    )
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Opaque2dBinKey {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.pipeline, state);
            ::core::hash::Hash::hash(&self.draw_function, state);
            ::core::hash::Hash::hash(&self.asset_id, state);
            ::core::hash::Hash::hash(&self.material_bind_group_id, state)
        }
    }
    impl PhaseItemBinKey for Opaque2dBinKey {
        type BatchSetKey = ();
        fn get_batch_set_key(&self) -> Option<Self::BatchSetKey> {
            None
        }
    }
    impl PhaseItem for Opaque2d {
        #[inline]
        fn entity(&self) -> Entity {
            self.representative_entity.0
        }
        fn main_entity(&self) -> MainEntity {
            self.representative_entity.1
        }
        #[inline]
        fn draw_function(&self) -> DrawFunctionId {
            self.key.draw_function
        }
        #[inline]
        fn batch_range(&self) -> &Range<u32> {
            &self.batch_range
        }
        #[inline]
        fn batch_range_mut(&mut self) -> &mut Range<u32> {
            &mut self.batch_range
        }
        fn extra_index(&self) -> PhaseItemExtraIndex {
            self.extra_index.clone()
        }
        fn batch_range_and_extra_index_mut(
            &mut self,
        ) -> (&mut Range<u32>, &mut PhaseItemExtraIndex) {
            (&mut self.batch_range, &mut self.extra_index)
        }
    }
    impl BinnedPhaseItem for Opaque2d {
        type BinKey = Opaque2dBinKey;
        fn new(
            key: Self::BinKey,
            representative_entity: (Entity, MainEntity),
            batch_range: Range<u32>,
            extra_index: PhaseItemExtraIndex,
        ) -> Self {
            Opaque2d {
                key,
                representative_entity,
                batch_range,
                extra_index,
            }
        }
    }
    impl CachedRenderPipelinePhaseItem for Opaque2d {
        #[inline]
        fn cached_pipeline(&self) -> CachedRenderPipelineId {
            self.key.pipeline
        }
    }
    /// Alpha mask 2D [`BinnedPhaseItem`]s.
    pub struct AlphaMask2d {
        /// The key, which determines which can be batched.
        pub key: AlphaMask2dBinKey,
        /// An entity from which data will be fetched, including the mesh if
        /// applicable.
        pub representative_entity: (Entity, MainEntity),
        /// The ranges of instances.
        pub batch_range: Range<u32>,
        /// An extra index, which is either a dynamic offset or an index in the
        /// indirect parameters list.
        pub extra_index: PhaseItemExtraIndex,
    }
    /// Data that must be identical in order to batch phase items together.
    pub struct AlphaMask2dBinKey {
        /// The identifier of the render pipeline.
        pub pipeline: CachedRenderPipelineId,
        /// The function used to draw.
        pub draw_function: DrawFunctionId,
        /// The asset that this phase item is associated with.
        ///
        /// Normally, this is the ID of the mesh, but for non-mesh items it might be
        /// the ID of another type of asset.
        pub asset_id: UntypedAssetId,
        /// The ID of a bind group specific to the material.
        pub material_bind_group_id: Option<BindGroupId>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AlphaMask2dBinKey {
        #[inline]
        fn clone(&self) -> AlphaMask2dBinKey {
            AlphaMask2dBinKey {
                pipeline: ::core::clone::Clone::clone(&self.pipeline),
                draw_function: ::core::clone::Clone::clone(&self.draw_function),
                asset_id: ::core::clone::Clone::clone(&self.asset_id),
                material_bind_group_id: ::core::clone::Clone::clone(
                    &self.material_bind_group_id,
                ),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AlphaMask2dBinKey {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AlphaMask2dBinKey {
        #[inline]
        fn eq(&self, other: &AlphaMask2dBinKey) -> bool {
            self.pipeline == other.pipeline && self.draw_function == other.draw_function
                && self.asset_id == other.asset_id
                && self.material_bind_group_id == other.material_bind_group_id
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for AlphaMask2dBinKey {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<CachedRenderPipelineId>;
            let _: ::core::cmp::AssertParamIsEq<DrawFunctionId>;
            let _: ::core::cmp::AssertParamIsEq<UntypedAssetId>;
            let _: ::core::cmp::AssertParamIsEq<Option<BindGroupId>>;
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for AlphaMask2dBinKey {
        #[inline]
        fn partial_cmp(
            &self,
            other: &AlphaMask2dBinKey,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.pipeline, &other.pipeline) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(
                        &self.draw_function,
                        &other.draw_function,
                    ) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                &self.asset_id,
                                &other.asset_id,
                            ) {
                                ::core::option::Option::Some(
                                    ::core::cmp::Ordering::Equal,
                                ) => {
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &self.material_bind_group_id,
                                        &other.material_bind_group_id,
                                    )
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for AlphaMask2dBinKey {
        #[inline]
        fn cmp(&self, other: &AlphaMask2dBinKey) -> ::core::cmp::Ordering {
            match ::core::cmp::Ord::cmp(&self.pipeline, &other.pipeline) {
                ::core::cmp::Ordering::Equal => {
                    match ::core::cmp::Ord::cmp(
                        &self.draw_function,
                        &other.draw_function,
                    ) {
                        ::core::cmp::Ordering::Equal => {
                            match ::core::cmp::Ord::cmp(
                                &self.asset_id,
                                &other.asset_id,
                            ) {
                                ::core::cmp::Ordering::Equal => {
                                    ::core::cmp::Ord::cmp(
                                        &self.material_bind_group_id,
                                        &other.material_bind_group_id,
                                    )
                                }
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for AlphaMask2dBinKey {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.pipeline, state);
            ::core::hash::Hash::hash(&self.draw_function, state);
            ::core::hash::Hash::hash(&self.asset_id, state);
            ::core::hash::Hash::hash(&self.material_bind_group_id, state)
        }
    }
    impl PhaseItem for AlphaMask2d {
        #[inline]
        fn entity(&self) -> Entity {
            self.representative_entity.0
        }
        #[inline]
        fn main_entity(&self) -> MainEntity {
            self.representative_entity.1
        }
        #[inline]
        fn draw_function(&self) -> DrawFunctionId {
            self.key.draw_function
        }
        #[inline]
        fn batch_range(&self) -> &Range<u32> {
            &self.batch_range
        }
        #[inline]
        fn batch_range_mut(&mut self) -> &mut Range<u32> {
            &mut self.batch_range
        }
        fn extra_index(&self) -> PhaseItemExtraIndex {
            self.extra_index.clone()
        }
        fn batch_range_and_extra_index_mut(
            &mut self,
        ) -> (&mut Range<u32>, &mut PhaseItemExtraIndex) {
            (&mut self.batch_range, &mut self.extra_index)
        }
    }
    impl BinnedPhaseItem for AlphaMask2d {
        type BinKey = AlphaMask2dBinKey;
        fn new(
            key: Self::BinKey,
            representative_entity: (Entity, MainEntity),
            batch_range: Range<u32>,
            extra_index: PhaseItemExtraIndex,
        ) -> Self {
            AlphaMask2d {
                key,
                representative_entity,
                batch_range,
                extra_index,
            }
        }
    }
    impl PhaseItemBinKey for AlphaMask2dBinKey {
        type BatchSetKey = ();
        fn get_batch_set_key(&self) -> Option<Self::BatchSetKey> {
            None
        }
    }
    impl CachedRenderPipelinePhaseItem for AlphaMask2d {
        #[inline]
        fn cached_pipeline(&self) -> CachedRenderPipelineId {
            self.key.pipeline
        }
    }
    /// Transparent 2D [`SortedPhaseItem`]s.
    pub struct Transparent2d {
        pub sort_key: FloatOrd,
        pub entity: (Entity, MainEntity),
        pub pipeline: CachedRenderPipelineId,
        pub draw_function: DrawFunctionId,
        pub batch_range: Range<u32>,
        pub extra_index: PhaseItemExtraIndex,
    }
    impl PhaseItem for Transparent2d {
        #[inline]
        fn entity(&self) -> Entity {
            self.entity.0
        }
        #[inline]
        fn main_entity(&self) -> MainEntity {
            self.entity.1
        }
        #[inline]
        fn draw_function(&self) -> DrawFunctionId {
            self.draw_function
        }
        #[inline]
        fn batch_range(&self) -> &Range<u32> {
            &self.batch_range
        }
        #[inline]
        fn batch_range_mut(&mut self) -> &mut Range<u32> {
            &mut self.batch_range
        }
        #[inline]
        fn extra_index(&self) -> PhaseItemExtraIndex {
            self.extra_index.clone()
        }
        #[inline]
        fn batch_range_and_extra_index_mut(
            &mut self,
        ) -> (&mut Range<u32>, &mut PhaseItemExtraIndex) {
            (&mut self.batch_range, &mut self.extra_index)
        }
    }
    impl SortedPhaseItem for Transparent2d {
        type SortKey = FloatOrd;
        #[inline]
        fn sort_key(&self) -> Self::SortKey {
            self.sort_key
        }
        #[inline]
        fn sort(items: &mut [Self]) {
            radsort::sort_by_key(items, |item| item.sort_key().0);
        }
    }
    impl CachedRenderPipelinePhaseItem for Transparent2d {
        #[inline]
        fn cached_pipeline(&self) -> CachedRenderPipelineId {
            self.pipeline
        }
    }
    pub fn extract_core_2d_camera_phases(
        mut transparent_2d_phases: ResMut<ViewSortedRenderPhases<Transparent2d>>,
        mut opaque_2d_phases: ResMut<ViewBinnedRenderPhases<Opaque2d>>,
        mut alpha_mask_2d_phases: ResMut<ViewBinnedRenderPhases<AlphaMask2d>>,
        cameras_2d: Extract<Query<(RenderEntity, &Camera), With<Camera2d>>>,
        mut live_entities: Local<EntityHashSet>,
    ) {
        live_entities.clear();
        for (entity, camera) in &cameras_2d {
            if !camera.is_active {
                continue;
            }
            transparent_2d_phases.insert_or_clear(entity);
            opaque_2d_phases.insert_or_clear(entity, GpuPreprocessingMode::None);
            alpha_mask_2d_phases.insert_or_clear(entity, GpuPreprocessingMode::None);
            live_entities.insert(entity);
        }
        transparent_2d_phases
            .retain(|camera_entity, _| live_entities.contains(camera_entity));
        opaque_2d_phases
            .retain(|camera_entity, _| live_entities.contains(camera_entity));
        alpha_mask_2d_phases
            .retain(|camera_entity, _| live_entities.contains(camera_entity));
    }
    pub fn prepare_core_2d_depth_textures(
        mut commands: Commands,
        mut texture_cache: ResMut<TextureCache>,
        render_device: Res<RenderDevice>,
        transparent_2d_phases: Res<ViewSortedRenderPhases<Transparent2d>>,
        opaque_2d_phases: Res<ViewBinnedRenderPhases<Opaque2d>>,
        views_2d: Query<(Entity, &ExtractedCamera, &Msaa), (With<Camera2d>,)>,
    ) {
        let mut textures = <HashMap<_, _>>::default();
        for (view, camera, msaa) in &views_2d {
            if !opaque_2d_phases.contains_key(&view)
                || !transparent_2d_phases.contains_key(&view)
            {
                continue;
            }
            let Some(physical_target_size) = camera.physical_target_size else {
                continue;
            };
            let cached_texture = textures
                .entry(camera.target.clone())
                .or_insert_with(|| {
                    let size = Extent3d {
                        depth_or_array_layers: 1,
                        width: physical_target_size.x,
                        height: physical_target_size.y,
                    };
                    let descriptor = TextureDescriptor {
                        label: Some("view_depth_texture"),
                        size,
                        mip_level_count: 1,
                        sample_count: msaa.samples(),
                        dimension: TextureDimension::D2,
                        format: CORE_2D_DEPTH_FORMAT,
                        usage: TextureUsages::RENDER_ATTACHMENT,
                        view_formats: &[],
                    };
                    texture_cache.get(&render_device, descriptor)
                })
                .clone();
            commands
                .entity(view)
                .insert(ViewDepthTexture::new(cached_texture, Some(0.0)));
        }
    }
}
