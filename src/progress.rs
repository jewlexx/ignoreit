pub struct CounterProgress<T: Fn(usize, usize)> {
    current: usize,
    total: usize,
    callback: T,
}

impl<T: Fn(usize, usize)> CounterProgress<T> {
    pub fn new(total: usize, callback: T) -> Self {
        Self {
            current: 0,
            total,
            callback,
        }
    }

    pub fn tick(&mut self) {
        self.current += 1;
        (self.callback)(self.current, self.total);
    }
}

// impl<T: Fn(usize, usize)> git2::Progress for CounterProgress<T> {
//     fn update(&mut self, op_code: git2::ProgressUpdate) -> Result<(), git2::Error> {
//         match op_code {
//             git2::ProgressUpdate::ReceivePack(received, total) => {
//                 self.current = received;
//                 (self.callback)(self.current, self.total);
//             }
//             git2::ProgressUpdate::SendPack(sent, total) => {
//                 self.current = sent;
//                 (self.callback)(self.current, self.total);
//             }
//             git2::ProgressUpdate::Indexing(current, total) => {
//                 self.current = current;
//                 (self.callback)(self.current, self.total);
//             }
//             git2::ProgressUpdate::Rebase(current, total) => {
//                 self.current = current;
//                 (self.callback)(self.current, self.total);
//             }
//             _ => {}
//         }
//         Ok(())
//     }
//     fn reset(&mut self) {
//         self.current = 0;
//     }
// }
