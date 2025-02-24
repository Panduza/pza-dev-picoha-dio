// Print debug support
use crate::api_dio::{PicohaDioAnswer, PicohaDioRequest};
use core::fmt::{self};

impl fmt::Debug for PicohaDioRequest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PicohaDioRequest {{ r#type: {:?}, pin_num: {:?}, value: {:?} }}",
            self.r#type, self.pin_num, self.value
        )
    }
}

impl fmt::Debug for PicohaDioAnswer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PicohaDioAnswer {{ r#type: {:?}, value: {:?} }}",
            self.r#type, self.value
        )
    }
}
