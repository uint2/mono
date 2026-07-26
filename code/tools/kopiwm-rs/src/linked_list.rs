pub(crate) trait LinkedListNode {
    fn next<'a>(&'a self) -> Option<&'a Self>
    where
        Self: Sized;

    fn next_editor(&mut self) -> &mut Option<Self>
    where
        Self: Sized;

    fn next_mut<'a>(&'a mut self) -> Option<&'a mut Self>
    where
        Self: Sized,
    {
        self.next_editor().as_mut()
    }

    // fn set_next(&mut self, value: Self) {
    //     // self.next_mut();
    // }
}
