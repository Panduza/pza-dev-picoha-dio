#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct PicohaDioRequest<'a> {
    #[femtopb(enumeration, tag = 1)]
    pub r#type: ::femtopb::enumeration::EnumValue<RequestType>,
    #[femtopb(uint32, tag = 2)]
    pub pin_num: u32,
    #[femtopb(enumeration, tag = 3)]
    pub value: ::femtopb::enumeration::EnumValue<PinValue>,
    #[femtopb(enumeration, tag = 4)]
    pub direction: ::femtopb::enumeration::EnumValue<PinDirection>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct PicohaDioAnswer<'a> {
    #[femtopb(enumeration, tag = 1)]
    pub r#type: ::femtopb::enumeration::EnumValue<AnswerType>,
    #[femtopb(enumeration, tag = 2)]
    pub value: ::femtopb::enumeration::EnumValue<PinValue>,
    #[femtopb(enumeration, tag = 3)]
    pub direction: ::femtopb::enumeration::EnumValue<PinDirection>,
    #[femtopb(string, optional, tag = 4)]
    pub error_message: ::core::option::Option<&'a str>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::femtopb::Enumeration)]
#[repr(i32)]
#[derive(Default)]
pub enum RequestType {
    #[default]
    Ping = 0,
    SetPinDirection = 1,
    SetPinValue = 2,
    GetPinDirection = 3,
    GetPinValue = 4,
}
impl RequestType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    #[allow(dead_code)]
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Ping => "PING",
            Self::SetPinDirection => "SET_PIN_DIRECTION",
            Self::SetPinValue => "SET_PIN_VALUE",
            Self::GetPinDirection => "GET_PIN_DIRECTION",
            Self::GetPinValue => "GET_PIN_VALUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    #[allow(dead_code)]
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PING" => Some(Self::Ping),
            "SET_PIN_DIRECTION" => Some(Self::SetPinDirection),
            "SET_PIN_VALUE" => Some(Self::SetPinValue),
            "GET_PIN_DIRECTION" => Some(Self::GetPinDirection),
            "GET_PIN_VALUE" => Some(Self::GetPinValue),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::femtopb::Enumeration)]
#[repr(i32)]
#[derive(Default)]
pub enum PinValue {
    #[default]
    Low = 0,
    High = 1,
}
impl PinValue {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    #[allow(dead_code)]
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Low => "LOW",
            Self::High => "HIGH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    #[allow(dead_code)]
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOW" => Some(Self::Low),
            "HIGH" => Some(Self::High),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::femtopb::Enumeration)]
#[repr(i32)]
#[derive(Default)]
pub enum PinDirection {
    #[default]
    Input = 0,
    Output = 1,
}
impl PinDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    #[allow(dead_code)]
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Input => "INPUT",
            Self::Output => "OUTPUT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    #[allow(dead_code)]
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INPUT" => Some(Self::Input),
            "OUTPUT" => Some(Self::Output),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::femtopb::Enumeration)]
#[repr(i32)]
#[derive(Default)]
pub enum AnswerType {
    #[default]
    Success = 0,
    Failure = 1,
}
impl AnswerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    #[allow(dead_code)]
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Success => "SUCCESS",
            Self::Failure => "FAILURE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    #[allow(dead_code)]
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUCCESS" => Some(Self::Success),
            "FAILURE" => Some(Self::Failure),
            _ => None,
        }
    }
}
