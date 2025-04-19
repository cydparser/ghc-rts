#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(std::marker::PhantomData<T>, [T; 0]);

impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(std::marker::PhantomData, [])
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }

    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.as_ptr(), len) }
    }

    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.as_mut_ptr(), len) }
    }
}

impl<T> std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
