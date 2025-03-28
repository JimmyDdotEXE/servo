/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use euclid::RigidTransform3D;

use crate::{Hand, Input, JointFrame, Native};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "ipc", derive(serde::Serialize, serde::Deserialize))]
pub struct InputId(pub u32);

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "ipc", derive(serde::Serialize, serde::Deserialize))]
pub enum Handedness {
    None,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "ipc", derive(serde::Serialize, serde::Deserialize))]
pub enum TargetRayMode {
    Gaze,
    TrackedPointer,
    Screen,
    TransientPointer,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "ipc", derive(serde::Serialize, serde::Deserialize))]
pub struct InputSource {
    pub handedness: Handedness,
    pub target_ray_mode: TargetRayMode,
    pub id: InputId,
    pub supports_grip: bool,
    pub hand_support: Option<Hand<()>>,
    pub profiles: Vec<String>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "ipc", derive(serde::Serialize, serde::Deserialize))]
pub struct InputFrame {
    pub id: InputId,
    pub target_ray_origin: Option<RigidTransform3D<f32, Input, Native>>,
    pub grip_origin: Option<RigidTransform3D<f32, Input, Native>>,
    pub pressed: bool,
    pub hand: Option<Box<Hand<JointFrame>>>,
    pub squeezed: bool,
    pub button_values: Vec<f32>,
    pub axis_values: Vec<f32>,
    pub input_changed: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "ipc", derive(serde::Serialize, serde::Deserialize))]
pub enum SelectEvent {
    /// Selection started
    Start,
    /// Selection ended *without* it being a contiguous select event
    End,
    /// Selection ended *with* it being a contiguous select event
    Select,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "ipc", derive(serde::Serialize, serde::Deserialize))]
pub enum SelectKind {
    Select,
    Squeeze,
}
