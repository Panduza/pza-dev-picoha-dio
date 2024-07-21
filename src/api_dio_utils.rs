// Print debug support
use crate::{api_dio::PicohaDioRequest, print_debug_message};
use core::fmt::{self, Write};

// Message deserialization support
use femtopb::Message;

impl fmt::Debug for PicohaDioRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PicohaDioRequest {{ r#type: {:?}, pin_num: {:?}, value: {:?} }}",
            self.r#type, self.pin_num, self.value
        )
    }
}
