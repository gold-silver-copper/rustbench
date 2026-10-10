pub struct S<'a> { data: &'a mut String }

impl<'a> Drop for S<'a> {
    fn drop(&mut self) { self.data.push_str("being dropped"); }
}

fn demo<'a>(s: S<'a>) -> &'a mut String { let p = &mut *s.data; p }
