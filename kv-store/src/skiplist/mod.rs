mod arena;

trait Allocator {
    fn allocate(&self, alain: usize, size:usize) -> usize;
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
}