#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__srv__SpotMotion_Request() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__srv__SpotMotion_Request__init(msg: *mut SpotMotion_Request) -> bool;
    fn webots_spot_msgs__srv__SpotMotion_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpotMotion_Request>, size: usize) -> bool;
    fn webots_spot_msgs__srv__SpotMotion_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpotMotion_Request>);
    fn webots_spot_msgs__srv__SpotMotion_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpotMotion_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SpotMotion_Request>) -> bool;
}

// Corresponds to webots_spot_msgs__srv__SpotMotion_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpotMotion_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub override_: bool,

}



impl Default for SpotMotion_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__srv__SpotMotion_Request__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__srv__SpotMotion_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpotMotion_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__SpotMotion_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__SpotMotion_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__SpotMotion_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpotMotion_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpotMotion_Request where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/srv/SpotMotion_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__srv__SpotMotion_Request() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__srv__SpotMotion_Response() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__srv__SpotMotion_Response__init(msg: *mut SpotMotion_Response) -> bool;
    fn webots_spot_msgs__srv__SpotMotion_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpotMotion_Response>, size: usize) -> bool;
    fn webots_spot_msgs__srv__SpotMotion_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpotMotion_Response>);
    fn webots_spot_msgs__srv__SpotMotion_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpotMotion_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SpotMotion_Response>) -> bool;
}

// Corresponds to webots_spot_msgs__srv__SpotMotion_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpotMotion_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,

}



impl Default for SpotMotion_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__srv__SpotMotion_Response__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__srv__SpotMotion_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpotMotion_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__SpotMotion_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__SpotMotion_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__SpotMotion_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpotMotion_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpotMotion_Response where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/srv/SpotMotion_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__srv__SpotMotion_Response() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__srv__SpotHeight_Request() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__srv__SpotHeight_Request__init(msg: *mut SpotHeight_Request) -> bool;
    fn webots_spot_msgs__srv__SpotHeight_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpotHeight_Request>, size: usize) -> bool;
    fn webots_spot_msgs__srv__SpotHeight_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpotHeight_Request>);
    fn webots_spot_msgs__srv__SpotHeight_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpotHeight_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SpotHeight_Request>) -> bool;
}

// Corresponds to webots_spot_msgs__srv__SpotHeight_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpotHeight_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub height: f64,

}



impl Default for SpotHeight_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__srv__SpotHeight_Request__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__srv__SpotHeight_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpotHeight_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__SpotHeight_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__SpotHeight_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__SpotHeight_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpotHeight_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpotHeight_Request where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/srv/SpotHeight_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__srv__SpotHeight_Request() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__srv__SpotHeight_Response() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__srv__SpotHeight_Response__init(msg: *mut SpotHeight_Response) -> bool;
    fn webots_spot_msgs__srv__SpotHeight_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpotHeight_Response>, size: usize) -> bool;
    fn webots_spot_msgs__srv__SpotHeight_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpotHeight_Response>);
    fn webots_spot_msgs__srv__SpotHeight_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpotHeight_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SpotHeight_Response>) -> bool;
}

// Corresponds to webots_spot_msgs__srv__SpotHeight_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpotHeight_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,

}



impl Default for SpotHeight_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__srv__SpotHeight_Response__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__srv__SpotHeight_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpotHeight_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__SpotHeight_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__SpotHeight_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__SpotHeight_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpotHeight_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpotHeight_Response where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/srv/SpotHeight_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__srv__SpotHeight_Response() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__srv__BlockPose_Request() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__srv__BlockPose_Request__init(msg: *mut BlockPose_Request) -> bool;
    fn webots_spot_msgs__srv__BlockPose_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BlockPose_Request>, size: usize) -> bool;
    fn webots_spot_msgs__srv__BlockPose_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BlockPose_Request>);
    fn webots_spot_msgs__srv__BlockPose_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BlockPose_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BlockPose_Request>) -> bool;
}

// Corresponds to webots_spot_msgs__srv__BlockPose_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BlockPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub block: rosidl_runtime_rs::String,

}



impl Default for BlockPose_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__srv__BlockPose_Request__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__srv__BlockPose_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BlockPose_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__BlockPose_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__BlockPose_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__BlockPose_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BlockPose_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BlockPose_Request where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/srv/BlockPose_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__srv__BlockPose_Request() }
  }
}


#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__srv__BlockPose_Response() -> *const std::ffi::c_void;
}

#[link(name = "webots_spot_msgs__rosidl_generator_c")]
extern "C" {
    fn webots_spot_msgs__srv__BlockPose_Response__init(msg: *mut BlockPose_Response) -> bool;
    fn webots_spot_msgs__srv__BlockPose_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BlockPose_Response>, size: usize) -> bool;
    fn webots_spot_msgs__srv__BlockPose_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BlockPose_Response>);
    fn webots_spot_msgs__srv__BlockPose_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BlockPose_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BlockPose_Response>) -> bool;
}

// Corresponds to webots_spot_msgs__srv__BlockPose_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BlockPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub location: rosidl_runtime_rs::String,

}



impl Default for BlockPose_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !webots_spot_msgs__srv__BlockPose_Response__init(&mut msg as *mut _) {
        panic!("Call to webots_spot_msgs__srv__BlockPose_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BlockPose_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__BlockPose_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__BlockPose_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { webots_spot_msgs__srv__BlockPose_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BlockPose_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BlockPose_Response where Self: Sized {
  const TYPE_NAME: &'static str = "webots_spot_msgs/srv/BlockPose_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__webots_spot_msgs__srv__BlockPose_Response() }
  }
}






#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__webots_spot_msgs__srv__SpotMotion() -> *const std::ffi::c_void;
}

// Corresponds to webots_spot_msgs__srv__SpotMotion
#[allow(missing_docs, non_camel_case_types)]
pub struct SpotMotion;

impl rosidl_runtime_rs::Service for SpotMotion {
    type Request = SpotMotion_Request;
    type Response = SpotMotion_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__webots_spot_msgs__srv__SpotMotion() }
    }
}




#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__webots_spot_msgs__srv__SpotHeight() -> *const std::ffi::c_void;
}

// Corresponds to webots_spot_msgs__srv__SpotHeight
#[allow(missing_docs, non_camel_case_types)]
pub struct SpotHeight;

impl rosidl_runtime_rs::Service for SpotHeight {
    type Request = SpotHeight_Request;
    type Response = SpotHeight_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__webots_spot_msgs__srv__SpotHeight() }
    }
}




#[link(name = "webots_spot_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__webots_spot_msgs__srv__BlockPose() -> *const std::ffi::c_void;
}

// Corresponds to webots_spot_msgs__srv__BlockPose
#[allow(missing_docs, non_camel_case_types)]
pub struct BlockPose;

impl rosidl_runtime_rs::Service for BlockPose {
    type Request = BlockPose_Request;
    type Response = BlockPose_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__webots_spot_msgs__srv__BlockPose() }
    }
}


