#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__msg__Int32() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__msg__Int32__init(msg: *mut Int32) -> bool;
    fn webots_spot_msgs__msg__Int32__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Int32>, size: usize) -> bool;
    fn webots_spot_msgs__msg__Int32__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Int32>);
    fn webots_spot_msgs__msg__Int32__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Int32>, out_seq: *mut rosidl_runtime_rs::Sequence<Int32>) -> bool;
}

// Corresponds to webots_spot_msgs__msg__Int32
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Int32 {

    // This member is not documented.
    #[allow(missing_docs)]
    pub data: i32,

}



impl Default for Int32 {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__msg__Int32__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__msg__Int32__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Int32 {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__msg__Int32__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__msg__Int32__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__msg__Int32__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Int32 {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Int32 where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/msg/Int32";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__msg__Int32() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__msg__Legs() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__msg__Legs__init(msg: *mut Legs) -> bool;
    fn webots_spot_msgs__msg__Legs__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Legs>, size: usize) -> bool;
    fn webots_spot_msgs__msg__Legs__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Legs>);
    fn webots_spot_msgs__msg__Legs__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Legs>, out_seq: *mut rosidl_runtime_rs::Sequence<Legs>) -> bool;
}

// Corresponds to webots_spot_msgs__msg__Legs
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Front left leg
/// Front right leg
/// Rear left leg
/// Rear right leg

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Legs {

    // This member is not documented.
    #[allow(missing_docs)]
    pub leg: [f64; 12],

}



impl Default for Legs {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__msg__Legs__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__msg__Legs__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Legs {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__msg__Legs__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__msg__Legs__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__msg__Legs__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Legs {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Legs where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/msg/Legs";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__msg__Legs() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__msg__GaitInput() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__msg__GaitInput__init(msg: *mut GaitInput) -> bool;
    fn webots_spot_msgs__msg__GaitInput__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GaitInput>, size: usize) -> bool;
    fn webots_spot_msgs__msg__GaitInput__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GaitInput>);
    fn webots_spot_msgs__msg__GaitInput__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GaitInput>, out_seq: *mut rosidl_runtime_rs::Sequence<GaitInput>) -> bool;
}

// Corresponds to webots_spot_msgs__msg__GaitInput
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GaitInput {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub roll: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pitch: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub step_length: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lateral_fraction: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_rate: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub step_velocity: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub clearance_height: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub penetration_depth: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub swing_period: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_control: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_control_on: f64,

}



impl Default for GaitInput {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__msg__GaitInput__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__msg__GaitInput__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GaitInput {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__msg__GaitInput__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__msg__GaitInput__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__msg__GaitInput__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GaitInput {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GaitInput where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/msg/GaitInput";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__msg__GaitInput() }
  }
}


