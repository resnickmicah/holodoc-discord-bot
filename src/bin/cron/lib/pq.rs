#[derive(Default)]
struct PQMin<T>
where
    T: Default + PartialOrd,
{
    items: Vec<T>,
}

impl<T> PQMin<T>
where
    T: Default + PartialOrd,
{
    // Creation
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    // Standard methods
    fn insert(&mut self, item: T) {
        self.items.push(item);
    }

    fn get_next(&mut self) -> Option<T> {
        if !self.has_next() {
            return None;
        }

        let item = self.items.remove(self.last_item());
        Some(item)
    }

    //  Helper methods
    fn size(&self) -> usize {
        self.items.len()
    }

    fn last_item(&self) -> usize {
        self.items.len() - 1
    }

    fn peak_next(&mut self) -> Option<T> {
        unimplemented!()
    }

    fn has_next(&self) -> bool {
        self.last_item() > 0
    }

    fn exch(&mut self, a: usize, b: usize) {
        self.items.swap(a, b);
    }

    fn compare(&self, a: usize, b: usize) -> bool {
        self.items[a] > self.items[b]
    }

    // Extras
    fn change(&mut self, k: usize, item: T) {}

    fn delete(&mut self, k: usize) {}

    fn contains(&mut self, k: usize) {}

    fn is_empty(&mut self) -> bool {
        unimplemented!()
    }

    // Invariants
    fn swim(&mut self, k: usize) {}

    fn sink(&mut self, k: usize) {}
}
