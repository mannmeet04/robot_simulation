#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to webots_spot_msgs__msg__Int32

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Int32 {

    // This member is not documented.
    #[allow(missing_docs)]
    pub data: i32,

}



impl Default for Int32 {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Int32::default())
  }
}

impl rosidl_runtime_rs::Message for Int32 {
  type RmwMsg = super::msg::rmw::Int32;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        data: msg.data,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      data: msg.data,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      data: msg.data,
    }
  }
}


// Corresponds to webots_spot_msgs__msg__Legs
/// Front left leg
/// Front right leg
/// Rear left leg
/// Rear right leg

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Legs {

    // This member is not documented.
    #[allow(missing_docs)]
    pub leg: [f64; 12],

}



impl Default for Legs {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Legs::default())
  }
}

impl rosidl_runtime_rs::Message for Legs {
  type RmwMsg = super::msg::rmw::Legs;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        leg: msg.leg,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        leg: msg.leg,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      leg: msg.leg,
    }
  }
}


// Corresponds to webots_spot_msgs__msg__GaitInput

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::GaitInput::default())
  }
}

impl rosidl_runtime_rs::Message for GaitInput {
  type RmwMsg = super::msg::rmw::GaitInput;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        roll: msg.roll,
        pitch: msg.pitch,
        yaw: msg.yaw,
        step_length: msg.step_length,
        lateral_fraction: msg.lateral_fraction,
        yaw_rate: msg.yaw_rate,
        step_velocity: msg.step_velocity,
        clearance_height: msg.clearance_height,
        penetration_depth: msg.penetration_depth,
        swing_period: msg.swing_period,
        yaw_control: msg.yaw_control,
        yaw_control_on: msg.yaw_control_on,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      roll: msg.roll,
      pitch: msg.pitch,
      yaw: msg.yaw,
      step_length: msg.step_length,
      lateral_fraction: msg.lateral_fraction,
      yaw_rate: msg.yaw_rate,
      step_velocity: msg.step_velocity,
      clearance_height: msg.clearance_height,
      penetration_depth: msg.penetration_depth,
      swing_period: msg.swing_period,
      yaw_control: msg.yaw_control,
      yaw_control_on: msg.yaw_control_on,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      roll: msg.roll,
      pitch: msg.pitch,
      yaw: msg.yaw,
      step_length: msg.step_length,
      lateral_fraction: msg.lateral_fraction,
      yaw_rate: msg.yaw_rate,
      step_velocity: msg.step_velocity,
      clearance_height: msg.clearance_height,
      penetration_depth: msg.penetration_depth,
      swing_period: msg.swing_period,
      yaw_control: msg.yaw_control,
      yaw_control_on: msg.yaw_control_on,
    }
  }
}


