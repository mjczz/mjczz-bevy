### 为什么AudioPlayer能作为实现了Bundle trait传入

#### 使用示例

```rust
use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioPlayer::new(
        asset_server.load("sounds/Windless Slopes.ogg"),
    ));
}
```



#### AudioPlayer是一个Component
```rust
#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(PlaybackSettings)]
pub struct AudioPlayer<Source = AudioSource>(pub Handle<Source>)
where
    Source: Asset + Decodable;


```

#### commands.spawn()，接收一个实现了Bundle的参数
```rust
impl<'w, 's> Commands<'w, 's> {
    ///
    #[track_caller]
    pub fn spawn<T: Bundle>(&mut self, bundle: T) -> EntityCommands {
        let mut entity = self.spawn_empty();
        entity.insert(bundle);
        entity
    }
    /// 。。。。
}
```

#### Bevy 的设计是为了使得任何实现了 Component trait 的类型都可以直接用作 Bundle。
> 这通过为所有 Component 类型自动实现 Bundle 和 DynamicBundle traits 实现的。
> 以下是自动实现 Bundle 和 DynamicBundle traits 的代码示例：

```rust
// 为所有实现了 Component trait 的类型自动实现 Bundle trait
unsafe impl<C: Component> Bundle for C {
    fn component_ids(
        components: &mut Components,
        storages: &mut Storages,
        ids: &mut impl FnMut(ComponentId),
    ) {
        ids(components.register_component::<C>(storages));
    }

    unsafe fn from_components<T, F>(ctx: &mut T, func: &mut F) -> Self
    where
        F: for<'a> FnMut(&'a mut T) -> OwningPtr<'a>,
        Self: Sized,
    {
        let ptr = func(ctx);
        unsafe { ptr.read() }
    }

    fn register_required_components(
        components: &mut Components,
        storages: &mut Storages,
        required_components: &mut RequiredComponents,
    ) {
        let component_id = components.register_component::<C>(storages);
        <C as Component>::register_required_components(
            component_id,
            components,
            storages,
            required_components,
            0,
            &mut Vec::new(),
        );
    }

    fn get_component_ids(components: &Components, ids: &mut impl FnMut(Option<ComponentId>)) {
        ids(components.get_id(TypeId::of::<C>()));
    }
}

// 为所有实现了 Component trait 的类型自动实现 DynamicBundle trait
impl<C: Component> DynamicBundle for C {
    #[inline]
    fn get_components(self, func: &mut impl FnMut(StorageType, OwningPtr<'_>)) {
        OwningPtr::make(self, |ptr| func(C::STORAGE_TYPE, ptr));
    }
}
```


#### 总结
由于 Bevy 自动为实现了 Component trait 的类型实现了 Bundle 和 DynamicBundle traits，你可以直接将 AudioPlayer 组件传递给 commands.spawn 方法。这使得代码更加简洁和易读，同时保留了灵活性。


