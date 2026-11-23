
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__Stack_Goal() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__action__Stack_Goal__init(msg: *mut Stack_Goal) -> bool;
    fn webots_spot_msgs__action__Stack_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Stack_Goal>, size: usize) -> bool;
    fn webots_spot_msgs__action__Stack_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Stack_Goal>);
    fn webots_spot_msgs__action__Stack_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Stack_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<Stack_Goal>) -> bool;
}

// Corresponds to webots_spot_msgs__action__Stack_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Stack_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub block: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub location: rosidl_runtime_rs::String,

}



impl Default for Stack_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__action__Stack_Goal__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__action__Stack_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Stack_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Stack_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Stack_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/action/Stack_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__Stack_Goal() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__Stack_Result() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__action__Stack_Result__init(msg: *mut Stack_Result) -> bool;
    fn webots_spot_msgs__action__Stack_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Stack_Result>, size: usize) -> bool;
    fn webots_spot_msgs__action__Stack_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Stack_Result>);
    fn webots_spot_msgs__action__Stack_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Stack_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<Stack_Result>) -> bool;
}

// Corresponds to webots_spot_msgs__action__Stack_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Stack_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stacked: bool,

}



impl Default for Stack_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__action__Stack_Result__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__action__Stack_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Stack_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Stack_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Stack_Result where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/action/Stack_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__Stack_Result() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__Stack_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__action__Stack_Feedback__init(msg: *mut Stack_Feedback) -> bool;
    fn webots_spot_msgs__action__Stack_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Stack_Feedback>, size: usize) -> bool;
    fn webots_spot_msgs__action__Stack_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Stack_Feedback>);
    fn webots_spot_msgs__action__Stack_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Stack_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<Stack_Feedback>) -> bool;
}

// Corresponds to webots_spot_msgs__action__Stack_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Stack_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stacking: bool,

}



impl Default for Stack_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__action__Stack_Feedback__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__action__Stack_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Stack_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Stack_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Stack_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/action/Stack_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__Stack_Feedback() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__Stack_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__action__Stack_FeedbackMessage__init(msg: *mut Stack_FeedbackMessage) -> bool;
    fn webots_spot_msgs__action__Stack_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Stack_FeedbackMessage>, size: usize) -> bool;
    fn webots_spot_msgs__action__Stack_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Stack_FeedbackMessage>);
    fn webots_spot_msgs__action__Stack_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Stack_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<Stack_FeedbackMessage>) -> bool;
}

// Corresponds to webots_spot_msgs__action__Stack_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Stack_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::Stack_Feedback,

}



impl Default for Stack_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__action__Stack_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__action__Stack_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Stack_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Stack_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Stack_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/action/Stack_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__Stack_FeedbackMessage() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_Goal() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__action__PeakAndDetectObject_Goal__init(msg: *mut PeakAndDetectObject_Goal) -> bool;
    fn webots_spot_msgs__action__PeakAndDetectObject_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_Goal>, size: usize) -> bool;
    fn webots_spot_msgs__action__PeakAndDetectObject_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_Goal>);
    fn webots_spot_msgs__action__PeakAndDetectObject_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PeakAndDetectObject_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_Goal>) -> bool;
}

// Corresponds to webots_spot_msgs__action__PeakAndDetectObject_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PeakAndDetectObject_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub image: rosidl_runtime_rs::String,

}



impl Default for PeakAndDetectObject_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__action__PeakAndDetectObject_Goal__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__action__PeakAndDetectObject_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PeakAndDetectObject_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PeakAndDetectObject_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PeakAndDetectObject_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/action/PeakAndDetectObject_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_Goal() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_Result() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__action__PeakAndDetectObject_Result__init(msg: *mut PeakAndDetectObject_Result) -> bool;
    fn webots_spot_msgs__action__PeakAndDetectObject_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_Result>, size: usize) -> bool;
    fn webots_spot_msgs__action__PeakAndDetectObject_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_Result>);
    fn webots_spot_msgs__action__PeakAndDetectObject_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PeakAndDetectObject_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_Result>) -> bool;
}

// Corresponds to webots_spot_msgs__action__PeakAndDetectObject_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PeakAndDetectObject_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub object: rosidl_runtime_rs::String,

}



impl Default for PeakAndDetectObject_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__action__PeakAndDetectObject_Result__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__action__PeakAndDetectObject_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PeakAndDetectObject_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PeakAndDetectObject_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PeakAndDetectObject_Result where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/action/PeakAndDetectObject_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_Result() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__action__PeakAndDetectObject_Feedback__init(msg: *mut PeakAndDetectObject_Feedback) -> bool;
    fn webots_spot_msgs__action__PeakAndDetectObject_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_Feedback>, size: usize) -> bool;
    fn webots_spot_msgs__action__PeakAndDetectObject_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_Feedback>);
    fn webots_spot_msgs__action__PeakAndDetectObject_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PeakAndDetectObject_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_Feedback>) -> bool;
}

// Corresponds to webots_spot_msgs__action__PeakAndDetectObject_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PeakAndDetectObject_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: rosidl_runtime_rs::String,

}



impl Default for PeakAndDetectObject_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__action__PeakAndDetectObject_Feedback__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__action__PeakAndDetectObject_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PeakAndDetectObject_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PeakAndDetectObject_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PeakAndDetectObject_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/action/PeakAndDetectObject_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_Feedback() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__action__PeakAndDetectObject_FeedbackMessage__init(msg: *mut PeakAndDetectObject_FeedbackMessage) -> bool;
    fn webots_spot_msgs__action__PeakAndDetectObject_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_FeedbackMessage>, size: usize) -> bool;
    fn webots_spot_msgs__action__PeakAndDetectObject_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_FeedbackMessage>);
    fn webots_spot_msgs__action__PeakAndDetectObject_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PeakAndDetectObject_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_FeedbackMessage>) -> bool;
}

// Corresponds to webots_spot_msgs__action__PeakAndDetectObject_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PeakAndDetectObject_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::PeakAndDetectObject_Feedback,

}



impl Default for PeakAndDetectObject_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__action__PeakAndDetectObject_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__action__PeakAndDetectObject_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PeakAndDetectObject_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PeakAndDetectObject_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PeakAndDetectObject_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/action/PeakAndDetectObject_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_FeedbackMessage() }
  }
}




#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__Stack_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__action__Stack_SendGoal_Request__init(msg: *mut Stack_SendGoal_Request) -> bool;
    fn webots_spot_msgs__action__Stack_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Stack_SendGoal_Request>, size: usize) -> bool;
    fn webots_spot_msgs__action__Stack_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Stack_SendGoal_Request>);
    fn webots_spot_msgs__action__Stack_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Stack_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Stack_SendGoal_Request>) -> bool;
}

// Corresponds to webots_spot_msgs__action__Stack_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Stack_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::Stack_Goal,

}



impl Default for Stack_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__action__Stack_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__action__Stack_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Stack_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Stack_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Stack_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/action/Stack_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__Stack_SendGoal_Request() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__Stack_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__action__Stack_SendGoal_Response__init(msg: *mut Stack_SendGoal_Response) -> bool;
    fn webots_spot_msgs__action__Stack_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Stack_SendGoal_Response>, size: usize) -> bool;
    fn webots_spot_msgs__action__Stack_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Stack_SendGoal_Response>);
    fn webots_spot_msgs__action__Stack_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Stack_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Stack_SendGoal_Response>) -> bool;
}

// Corresponds to webots_spot_msgs__action__Stack_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Stack_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for Stack_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__action__Stack_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__action__Stack_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Stack_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Stack_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Stack_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/action/Stack_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__Stack_SendGoal_Response() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__Stack_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__action__Stack_GetResult_Request__init(msg: *mut Stack_GetResult_Request) -> bool;
    fn webots_spot_msgs__action__Stack_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Stack_GetResult_Request>, size: usize) -> bool;
    fn webots_spot_msgs__action__Stack_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Stack_GetResult_Request>);
    fn webots_spot_msgs__action__Stack_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Stack_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Stack_GetResult_Request>) -> bool;
}

// Corresponds to webots_spot_msgs__action__Stack_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Stack_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for Stack_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__action__Stack_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__action__Stack_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Stack_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Stack_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Stack_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/action/Stack_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__Stack_GetResult_Request() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__Stack_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__action__Stack_GetResult_Response__init(msg: *mut Stack_GetResult_Response) -> bool;
    fn webots_spot_msgs__action__Stack_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Stack_GetResult_Response>, size: usize) -> bool;
    fn webots_spot_msgs__action__Stack_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Stack_GetResult_Response>);
    fn webots_spot_msgs__action__Stack_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Stack_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Stack_GetResult_Response>) -> bool;
}

// Corresponds to webots_spot_msgs__action__Stack_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Stack_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::Stack_Result,

}



impl Default for Stack_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__action__Stack_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__action__Stack_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Stack_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__Stack_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Stack_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Stack_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/action/Stack_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__Stack_GetResult_Response() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Request__init(msg: *mut PeakAndDetectObject_SendGoal_Request) -> bool;
    fn webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_SendGoal_Request>, size: usize) -> bool;
    fn webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_SendGoal_Request>);
    fn webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PeakAndDetectObject_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_SendGoal_Request>) -> bool;
}

// Corresponds to webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PeakAndDetectObject_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::PeakAndDetectObject_Goal,

}



impl Default for PeakAndDetectObject_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PeakAndDetectObject_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PeakAndDetectObject_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PeakAndDetectObject_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/action/PeakAndDetectObject_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Request() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Response__init(msg: *mut PeakAndDetectObject_SendGoal_Response) -> bool;
    fn webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_SendGoal_Response>, size: usize) -> bool;
    fn webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_SendGoal_Response>);
    fn webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PeakAndDetectObject_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_SendGoal_Response>) -> bool;
}

// Corresponds to webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PeakAndDetectObject_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for PeakAndDetectObject_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PeakAndDetectObject_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PeakAndDetectObject_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PeakAndDetectObject_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/action/PeakAndDetectObject_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_SendGoal_Response() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__action__PeakAndDetectObject_GetResult_Request__init(msg: *mut PeakAndDetectObject_GetResult_Request) -> bool;
    fn webots_spot_msgs__action__PeakAndDetectObject_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_GetResult_Request>, size: usize) -> bool;
    fn webots_spot_msgs__action__PeakAndDetectObject_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_GetResult_Request>);
    fn webots_spot_msgs__action__PeakAndDetectObject_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PeakAndDetectObject_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_GetResult_Request>) -> bool;
}

// Corresponds to webots_spot_msgs__action__PeakAndDetectObject_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PeakAndDetectObject_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for PeakAndDetectObject_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__action__PeakAndDetectObject_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__action__PeakAndDetectObject_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PeakAndDetectObject_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PeakAndDetectObject_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PeakAndDetectObject_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/action/PeakAndDetectObject_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_GetResult_Request() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__action__PeakAndDetectObject_GetResult_Response__init(msg: *mut PeakAndDetectObject_GetResult_Response) -> bool;
    fn webots_spot_msgs__action__PeakAndDetectObject_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_GetResult_Response>, size: usize) -> bool;
    fn webots_spot_msgs__action__PeakAndDetectObject_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_GetResult_Response>);
    fn webots_spot_msgs__action__PeakAndDetectObject_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PeakAndDetectObject_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PeakAndDetectObject_GetResult_Response>) -> bool;
}

// Corresponds to webots_spot_msgs__action__PeakAndDetectObject_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PeakAndDetectObject_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::PeakAndDetectObject_Result,

}



impl Default for PeakAndDetectObject_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__action__PeakAndDetectObject_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__action__PeakAndDetectObject_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PeakAndDetectObject_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__action__PeakAndDetectObject_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PeakAndDetectObject_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PeakAndDetectObject_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/action/PeakAndDetectObject_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_GetResult_Response() }
  }
}






#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__webots_spot_msgs__action__Stack_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to webots_spot_msgs__action__Stack_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct Stack_SendGoal;

impl rosidl_runtime_rs::Service for Stack_SendGoal {
    type Request = Stack_SendGoal_Request;
    type Response = Stack_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__webots_spot_msgs__action__Stack_SendGoal() }
    }
}




#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__webots_spot_msgs__action__Stack_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to webots_spot_msgs__action__Stack_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct Stack_GetResult;

impl rosidl_runtime_rs::Service for Stack_GetResult {
    type Request = Stack_GetResult_Request;
    type Response = Stack_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__webots_spot_msgs__action__Stack_GetResult() }
    }
}




#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to webots_spot_msgs__action__PeakAndDetectObject_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct PeakAndDetectObject_SendGoal;

impl rosidl_runtime_rs::Service for PeakAndDetectObject_SendGoal {
    type Request = PeakAndDetectObject_SendGoal_Request;
    type Response = PeakAndDetectObject_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_SendGoal() }
    }
}




#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to webots_spot_msgs__action__PeakAndDetectObject_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct PeakAndDetectObject_GetResult;

impl rosidl_runtime_rs::Service for PeakAndDetectObject_GetResult {
    type Request = PeakAndDetectObject_GetResult_Request;
    type Response = PeakAndDetectObject_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__webots_spot_msgs__action__PeakAndDetectObject_GetResult() }
    }
}


