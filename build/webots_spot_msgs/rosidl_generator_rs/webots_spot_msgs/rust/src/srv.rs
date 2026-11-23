#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to webots_spot_msgs__srv__SpotMotion_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpotMotion_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub override_: bool,

}



impl Default for SpotMotion_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SpotMotion_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SpotMotion_Request {
  type RmwMsg = super::srv::rmw::SpotMotion_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        override_: msg.override_,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      override_: msg.override_,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      override_: msg.override_,
    }
  }
}


// Corresponds to webots_spot_msgs__srv__SpotMotion_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpotMotion_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,

}



impl Default for SpotMotion_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SpotMotion_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SpotMotion_Response {
  type RmwMsg = super::srv::rmw::SpotMotion_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      answer: msg.answer.to_string(),
    }
  }
}


// Corresponds to webots_spot_msgs__srv__SpotHeight_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpotHeight_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub height: f64,

}



impl Default for SpotHeight_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SpotHeight_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SpotHeight_Request {
  type RmwMsg = super::srv::rmw::SpotHeight_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        height: msg.height,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      height: msg.height,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      height: msg.height,
    }
  }
}


// Corresponds to webots_spot_msgs__srv__SpotHeight_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpotHeight_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,

}



impl Default for SpotHeight_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SpotHeight_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SpotHeight_Response {
  type RmwMsg = super::srv::rmw::SpotHeight_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      answer: msg.answer.to_string(),
    }
  }
}


// Corresponds to webots_spot_msgs__srv__BlockPose_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BlockPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub block: std::string::String,

}



impl Default for BlockPose_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::BlockPose_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BlockPose_Request {
  type RmwMsg = super::srv::rmw::BlockPose_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        block: msg.block.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        block: msg.block.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      block: msg.block.to_string(),
    }
  }
}


// Corresponds to webots_spot_msgs__srv__BlockPose_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BlockPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub location: std::string::String,

}



impl Default for BlockPose_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::BlockPose_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BlockPose_Response {
  type RmwMsg = super::srv::rmw::BlockPose_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        location: msg.location.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        location: msg.location.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      location: msg.location.to_string(),
    }
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


