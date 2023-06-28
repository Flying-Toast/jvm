use crate::Marker;

pub unsafe trait Trace {
    fn trace(&self, m: &mut Marker);
}

macro_rules! UNSAFE_noop_trace_impl {
    ($($t:ty),+$(,)?) => {
        $(
            unsafe impl $crate::Trace for $t {
                fn trace(&self, _: &mut $crate::Marker) {}
            }
        )+
    }
}

unsafe impl<T: Trace> Trace for Option<T> {
    fn trace(&self, m: &mut Marker) {
        match self {
            None => {}
            Some(o) => o.trace(m),
        }
    }
}

unsafe impl<T: Trace> Trace for Vec<T> {
    fn trace(&self, m: &mut Marker) {
        for x in self {
            x.trace(m);
        }
    }
}

UNSAFE_noop_trace_impl!(
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    i128,
    u128,
    bool,
    char,
    f32,
    f64,
    (),
    String,
);
