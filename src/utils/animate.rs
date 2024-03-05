use bevy::prelude::*;
use interpolation::{Ease, EaseFunction};
use std::{ops::Add, time::Duration};

#[derive(Component)]
pub struct AniTimer {
    duration: Duration,
    progress: f32,
    times: f32,
    max_times: f32,
    finish: bool,
}

impl Default for AniTimer {
    fn default() -> Self {
        Self {
            duration: Duration::ZERO,
            progress: 0.,
            times: 0.,
            max_times: 1.,
            finish: false,
        }
    }
}

impl AniTimer {
    fn reset(&mut self) {
        self.progress = 0.;
        self.times = 0.;
        self.finish = false;
    }

    fn tick(&mut self, delta: Duration) {
        if self.times == self.max_times || self.times + self.progress == self.max_times {
            self.finish = true;
        }
        if self.duration == Duration::ZERO {
            return;
        }
        self.progress += delta.as_secs_f32() / self.duration.as_secs_f32();
        if self.progress >= 1.0 {
            self.progress = 0.0;
            self.times += 1.0;
        }
        if self.times + self.progress >= self.max_times {
            self.times = self.max_times.trunc();
            let frc = self.max_times - self.times;
            self.progress = if frc == 0.0 { 1. } else { frc };
        }
    }

    pub fn set_repeat(&mut self, repeat: Repeat) {
        self.max_times = match repeat {
            Repeat::Finite(n) => n as f32,
            Repeat::Infinite => f32::MAX,
            Repeat::Duration(d) => d.as_secs_f32() / self.duration.as_secs_f32(),
        }
    }
}

pub enum Repeat {
    Finite(u32),
    Infinite,
    Duration(Duration),
}

impl Default for Repeat {
    fn default() -> Self {
        Self::Finite(1)
    }
}

trait Animate<T> {
    fn copy_from(&mut self, other: &T);
}

#[derive(Clone, Copy)]
pub struct AniSprite {
    pub color: Color,
    pub size: Vec2,
}

impl Animate<Sprite> for AniSprite {
    fn copy_from(&mut self, other: &Sprite) {
        self.color = other.color;
        if let Some(size) = other.custom_size {
            self.size = size;
        }
    }
}

impl Add for AniSprite {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            color: self.color + rhs.color,
            size: self.size + rhs.size,
        }
    }
}

impl Default for AniSprite {
    fn default() -> Self {
        Self {
            color: Color::NONE,
            size: Vec2::ZERO,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct AniStyle {
    pub width: Val,
    pub height: Val,
    pub left: Val,
    pub right: Val,
    pub top: Val,
    pub bottom: Val,
}

impl Animate<Style> for AniStyle {
    fn copy_from(&mut self, other: &Style) {
        self.width = other.width;
        self.height = other.height;
        self.left = other.left;
        self.right = other.right;
        self.top = other.top;
        self.bottom = other.bottom;
    }
}

fn val_add(a: Val, b: Val) -> Val {
    match (a, b) {
        (Val::Px(a), Val::Px(b)) => Val::Px(a + b),
        (Val::Percent(a), Val::Percent(b)) => Val::Percent(a + b),
        _ => a,
    }
}

impl Add for AniStyle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            width: val_add(self.width, rhs.width),
            height: val_add(self.height, rhs.height),
            left: val_add(self.left, rhs.left),
            right: val_add(self.right, rhs.right),
            top: val_add(self.top, rhs.top),
            bottom: val_add(self.bottom, rhs.bottom),
        }
    }
}

#[derive(Clone, Copy)]
pub struct AniTransform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for AniTransform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::from_vec4(Vec4::ZERO),
            scale: Vec3::ZERO,
        }
    }
}

impl Animate<Transform> for AniTransform {
    fn copy_from(&mut self, other: &Transform) {
        self.translation = other.translation;
        self.rotation = other.rotation;
        self.scale = other.scale;
    }
}

impl Add for AniTransform {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            translation: self.translation + rhs.translation,
            rotation: self.rotation * rhs.rotation,
            scale: self.scale + rhs.scale,
        }
    }
}

pub struct AniText {
    pub text: String,
    pub font_size: f32,
    pub color: Color,
}

impl AniText {
    pub fn set_text(&mut self, text: impl Into<String>) -> &mut Self {
        self.text = text.into();
        self
    }

    pub fn set_font_size(&mut self, font_size: f32) -> &mut Self {
        self.font_size = font_size;
        self
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }
}

impl Default for AniText {
    fn default() -> Self {
        Self {
            text: String::new(),
            font_size: 0.0,
            color: Color::NONE,
        }
    }
}

impl Animate<Text> for Vec<AniText> {
    fn copy_from(&mut self, other: &Text) {
        for i in 0..self.len() {
            self[i].text = other.sections[i].value.clone();
            self[i].font_size = other.sections[i].style.font_size;
            self[i].color = other.sections[i].style.color;
        }
        for i in self.len()..other.sections.len() {
            self.push(AniText::default());
            self[i].text = other.sections[i].value.clone();
            self[i].font_size = other.sections[i].style.font_size;
            self[i].color = other.sections[i].style.color;
        }
        /* for (index, section) in other.sections.iter().enumerate() {
            self[index] = AniText::default();
            self[index].text = section.value.clone();
            self[index].font_size = section.style.font_size;
            self[index].color = section.style.color;
        } */
    }
}

impl Add for AniText {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            text: self.text + &rhs.text,
            font_size: self.font_size + rhs.font_size,
            color: self.color + rhs.color,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum EaseMethod {
    EaseFn(EaseFunction),
    #[default]
    Linear,
    Custom(fn(f32) -> f32),
}

impl EaseMethod {
    pub fn tick(&self, tick: f32) -> f32 {
        match self {
            EaseMethod::EaseFn(fun) => tick.calc(*fun),
            EaseMethod::Linear => tick,
            EaseMethod::Custom(fun) => fun(tick),
        }
    }
}

impl Into<EaseMethod> for EaseFunction {
    fn into(self) -> EaseMethod {
        EaseMethod::EaseFn(self)
    }
}

#[derive(Default)]
pub struct Animation {
    timer: AniTimer,
    change_sprite: AniSprite,
    change_style: AniStyle,
    change_transform: AniTransform,
    change_text: Vec<AniText>,
    ease_method: EaseMethod,
    retain_change: bool,
}

#[derive(Default)]
pub enum AniType {
    Sprite(AniSprite, AniSprite),
    Style(AniStyle, AniStyle),
    Transform(AniTransform, AniTransform),
    #[default]
    Delay,
}

fn lerp_color(start: Color, change: Color, ratio: f32) -> Color {
    /* let s: Vec4 = start.rgba_to_vec4();
    let c: Vec4 = change.rgba_to_vec4();
    Vec4::new(
        s.x + c.x * ratio,
        s.y + c.y * ratio,
        s.z + c.z * ratio,
        s.w + c.w * ratio,
    ) */
    start + change * ratio
}

fn lerp_val(start: Val, change: Val, ratio: f32) -> Val {
    match (start, change) {
        (Val::Px(s), Val::Px(c)) => Val::Px(s + c * ratio),
        (Val::Percent(s), Val::Percent(c)) => Val::Percent(s + c * ratio),
        _ => start,
    }
}

impl Animation {
    pub fn set_retain_change(&mut self, retain: bool) -> &mut Self {
        self.retain_change = retain;
        self
    }

    pub fn set_repeat(&mut self, repeat: Repeat) -> &mut Self {
        self.timer.set_repeat(repeat);
        self
    }

    pub fn set_duration(&mut self, duration: Duration) -> &mut Self {
        self.timer.duration = duration;
        self
    }

    pub fn set_ease(&mut self, ease: EaseMethod) -> &mut Self {
        self.ease_method = ease;
        self
    }

    pub fn set_sprite(&mut self, size: Vec2, color: Color) -> &mut Self {
        self.change_sprite.size = size;
        self.change_sprite.color = color;
        self
    }

    pub fn set_sprite_size(&mut self, size: Vec2) -> &mut Self {
        self.change_sprite.size = size;
        self
    }

    pub fn set_sprite_color(&mut self, color: Color) -> &mut Self {
        self.change_sprite.color = color;
        self
    }

    pub fn set_style(
        &mut self,
        width: Val,
        height: Val,
        left: Val,
        right: Val,
        top: Val,
        bottom: Val,
    ) -> &mut Self {
        self.change_style.width = width;
        self.change_style.height = height;
        self.change_style.left = left;
        self.change_style.right = right;
        self.change_style.top = top;
        self.change_style.bottom = bottom;
        self
    }

    pub fn set_style_size(&mut self, width: Val, height: Val) -> &mut Self {
        self.change_style.width = width;
        self.change_style.height = height;
        self
    }

    pub fn set_style_margin(&mut self, left: Val, right: Val, top: Val, bottom: Val) -> &mut Self {
        self.change_style.left = left;
        self.change_style.right = right;
        self.change_style.top = top;
        self.change_style.bottom = bottom;
        self
    }

    pub fn set_style_margin_all(&mut self, margin: Val) -> &mut Self {
        self.change_style.left = margin;
        self.change_style.right = margin;
        self.change_style.top = margin;
        self.change_style.bottom = margin;
        self
    }

    pub fn set_style_margin_x(&mut self, x: Val) -> &mut Self {
        self.change_style.left = x;
        self.change_style.right = x;
        self
    }

    pub fn set_style_margin_y(&mut self, y: Val) -> &mut Self {
        self.change_style.top = y;
        self.change_style.bottom = y;
        self
    }

    pub fn set_style_left(&mut self, left: Val) -> &mut Self {
        self.change_style.left = left;
        self
    }

    pub fn set_style_right(&mut self, right: Val) -> &mut Self {
        self.change_style.right = right;
        self
    }

    pub fn set_style_top(&mut self, top: Val) -> &mut Self {
        self.change_style.top = top;
        self
    }

    pub fn set_style_bottom(&mut self, bottom: Val) -> &mut Self {
        self.change_style.bottom = bottom;
        self
    }

    pub fn set_transform(&mut self, translation: Vec3, rotation: Quat, scale: Vec3) -> &mut Self {
        self.change_transform.translation = translation;
        self.change_transform.rotation = rotation;
        self.change_transform.scale = scale;
        self
    }

    pub fn set_transform_translation(&mut self, translation: Vec3) -> &mut Self {
        self.change_transform.translation = translation;
        self
    }

    pub fn set_transform_rotation(&mut self, rotation: Quat) -> &mut Self {
        self.change_transform.rotation = rotation;
        self
    }

    pub fn set_transform_scale(&mut self, scale: Vec3) -> &mut Self {
        self.change_transform.scale = scale;
        self
    }

    pub fn add_text(&mut self) -> &mut AniText {
        self.change_text.push(AniText::default());
        self.change_text.last_mut().unwrap()
    }
}

#[derive(Component)]
pub struct Animator {
    exec_index: usize,
    pause: bool,
    loop_strategy: LoopStrategy,
    start_sprite: AniSprite,
    start_style: AniStyle,
    start_transform: AniTransform,
    start_text: Vec<AniText>,
    init_sprite: bool,
    init_transform: bool,
    init_style: bool,
    init_text: bool,
    animations: Vec<Animation>,
    len: usize,
}

impl Default for Animator {
    fn default() -> Self {
        Self {
            exec_index: 0,
            pause: false,
            loop_strategy: LoopStrategy::default(),
            start_sprite: AniSprite::default(),
            start_style: AniStyle::default(),
            start_transform: AniTransform::default(),
            start_text: Vec::new(),
            init_sprite: true,
            init_transform: true,
            init_style: true,
            init_text: true,
            animations: Vec::new(),
            len: 0,
        }
    }
}

impl Animator {
    fn tick(&mut self, delta: Duration) {
        if let Some(animation) = self.animations.get_mut(self.exec_index) {
            if animation.timer.finish {
                if animation.retain_change {
                    self.start_sprite = self.start_sprite + animation.change_sprite;
                    self.start_style = self.start_style + animation.change_style;
                    self.start_transform = self.start_transform + animation.change_transform;
                }
                match self.loop_strategy {
                    LoopStrategy::AllOnce => {
                        self.exec_index = (self.exec_index + 1).min(self.len);
                    }
                    LoopStrategy::LoopAll => {
                        self.exec_index = (self.exec_index + 1) % self.len;
                        self.reset_all();
                        self.tick(delta);
                    }
                    LoopStrategy::LoopOne => {
                        animation.timer.reset();
                        self.tick(delta);
                    }
                    LoopStrategy::Once => {
                        self.pause = true;
                    }
                }
            } else {
                animation.timer.tick(delta);
            }
        }
    }

    fn tick_sprite(&mut self, sprite: &mut Sprite) {
        if self.init_sprite {
            self.start_sprite.copy_from(sprite);
            self.init_sprite = false;
        }
        if let Some(animation) = self.animations.get_mut(self.exec_index) {
            let ratio = animation.ease_method.tick(animation.timer.progress);
            if animation.change_sprite.size != Vec2::ZERO {
                if let Some(size) = sprite.custom_size.as_mut() {
                    size.x = self.start_sprite.size.x + animation.change_sprite.size.x * ratio;
                    size.y = self.start_sprite.size.y + animation.change_sprite.size.y * ratio;
                }
            }
            if animation.change_sprite.color != Color::NONE {
                sprite.color = lerp_color(
                    self.start_sprite.color,
                    animation.change_sprite.color,
                    ratio,
                );
            }
        }
    }

    fn tick_style(&mut self, style: &mut Style) {
        if self.init_style {
            self.start_style.copy_from(style);
            self.init_style = false;
        }
        if let Some(animation) = self.animations.get_mut(self.exec_index) {
            let ratio = animation.ease_method.tick(animation.timer.progress);
            if animation.change_style.width != Val::Auto {
                style.width = lerp_val(self.start_style.width, animation.change_style.width, ratio);
            }
            if animation.change_style.height != Val::Auto {
                style.height = lerp_val(
                    self.start_style.height,
                    animation.change_style.height,
                    ratio,
                );
            }
            if animation.change_style.left != Val::Auto {
                style.left = lerp_val(self.start_style.left, animation.change_style.left, ratio);
            }
            if animation.change_style.right != Val::Auto {
                style.right = lerp_val(self.start_style.right, animation.change_style.right, ratio);
            }
            if animation.change_style.top != Val::Auto {
                style.top = lerp_val(self.start_style.top, animation.change_style.top, ratio);
            }
            if animation.change_style.bottom != Val::Auto {
                style.bottom = lerp_val(
                    self.start_style.bottom,
                    animation.change_style.bottom,
                    ratio,
                );
            }
        }
    }

    fn tick_transform(&mut self, transform: &mut Transform) {
        if self.init_transform {
            self.start_transform.copy_from(transform);
            self.init_transform = false;
        }
        if let Some(animation) = self.animations.get_mut(self.exec_index) {
            let ratio = animation.ease_method.tick(animation.timer.progress);
            if animation.change_transform.translation != Vec3::ZERO {
                transform.translation = self.start_transform.translation
                    + animation.change_transform.translation * ratio;
            }
            if animation.change_transform.rotation != Quat::from_vec4(Vec4::ZERO) {
                transform.rotation = self.start_transform.rotation.slerp(
                    self.start_transform.rotation + animation.change_transform.rotation,
                    ratio,
                );
            }
            if animation.change_transform.scale != Vec3::ZERO {
                transform.scale =
                    self.start_transform.scale + animation.change_transform.scale * ratio;
            }
        }
    }

    fn tick_text(&mut self, text: &mut Text) {
        if self.init_text {
            self.start_text.copy_from(text);
            self.init_text = false;
        }
        if let Some(animation) = self.animations.get_mut(self.exec_index) {
            let ratio = animation.ease_method.tick(animation.timer.progress);
            for (index, txt) in animation.change_text.iter().enumerate() {
                if txt.text != text.sections[index].value {
                    text.sections[index].value = txt.text.clone();
                }
                if txt.font_size != 0.0 {
                    text.sections[index].style.font_size =
                        self.start_text[index].font_size + txt.font_size * ratio;
                }
                if txt.color != Color::NONE {
                    text.sections[index].style.color =
                        lerp_color(self.start_text[index].color, txt.color, ratio);
                }
            }
        }
    }

    fn reset_all(&mut self) {
        for animation in self.animations.iter_mut() {
            animation.timer.reset();
        }
    }

    pub fn get_animations_len(&self) -> usize {
        self.len
    }

    pub fn get_exec_index(&self) -> usize {
        self.exec_index
    }

    pub fn exec_index(&mut self, index: usize) -> &mut Self {
        if let Some(animation) = self.animations.get_mut(self.exec_index) {
            animation.timer.reset();
            self.exec_index = index;
            self.pause = false;
        }
        self
    }

    pub fn exec_next(&mut self) -> &mut Self {
        self.exec_index += 1;
        self.pause = false;
        self
    }

    pub fn get_pause(&self) -> bool {
        self.pause
    }

    pub fn get_is_finish(&self) -> bool {
        self.animations
            .get(self.exec_index)
            .map_or(true, |ani| ani.timer.finish)
    }

    pub fn add_change(&mut self) -> &mut Animation {
        self.animations.push(Animation::default());
        self.len += 1;
        self.animations.last_mut().unwrap()
    }

    pub fn retain_transform(&mut self) -> &mut Self {
        self.init_transform = true;
        self
    }

    pub fn retain_style(&mut self) -> &mut Self {
        self.init_style = true;
        self
    }

    pub fn retain_sprite(&mut self) -> &mut Self {
        self.init_sprite = true;
        self
    }

    pub fn set_loop_strategy(&mut self, strategy: LoopStrategy) -> &mut Self {
        self.loop_strategy = strategy;
        self
    }

    pub fn set_pause(&mut self, pause: bool) -> &mut Self {
        self.pause = pause;
        self
    }

    pub fn set_exec_index(&mut self, index: usize) -> &mut Self {
        self.exec_index = index;
        self
    }
}

#[derive(Default)]
pub enum LoopStrategy {
    #[default]
    AllOnce,
    LoopAll,
    LoopOne,
    Once,
}

pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_transform);
        #[cfg(feature = "bevy_ui")]
        app.add_systems(Update, update_style);
        #[cfg(feature = "bevy_sprite")]
        app.add_systems(Update, update_sprite);
        #[cfg(feature = "bevy_text")]
        app.add_systems(Update, update_text);
    }
}
#[cfg(feature = "bevy_ui")]
fn update_style(mut query: Query<(&mut Style, &mut Animator)>) {
    for (mut style, mut animator) in query.iter_mut() {
        if !animator.pause {
            animator.tick_style(&mut style);
        }
    }
}

fn update_transform(mut query: Query<(&mut Transform, &mut Animator)>, time: Res<Time>) {
    let delta = time.delta();
    for (mut transform, mut animator) in query.iter_mut() {
        if !animator.pause {
            animator.tick_transform(&mut transform);
            animator.tick(delta);
        }
    }
}
#[cfg(feature = "bevy_sprite")]
fn update_sprite(mut query: Query<(&mut Sprite, &mut Animator)>) {
    for (mut sprite, mut animator) in query.iter_mut() {
        if !animator.pause {
            animator.tick_sprite(&mut sprite);
        }
    }
}
#[cfg(feature = "bevy_text")]
fn update_text(mut query: Query<(&mut Text, &mut Animator)>) {
    for (mut text, mut animator) in query.iter_mut() {
        if !animator.pause {
            animator.tick_text(&mut text);
        }
    }
}
