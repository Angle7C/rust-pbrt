use std::sync::atomic::AtomicU32;

pub struct Block {
    pub blocks: Vec<(u32, u32)>,
    pub dim: (u32, u32),
    pub next: AtomicU32,
}
impl Block {
    pub fn new(img: (u32, u32), dim: (u32, u32)) -> Self {
        let num = (img.0 / dim.0, img.1 / dim.1);
        let mut blocks: Vec<_> = (0..num.0 * num.1).map(|i| (i % num.0, i / num.0)).collect();
        blocks.sort_by(|a, b| (a.0 * dim.0 + a.1 * dim.1).cmp(&(b.0 * dim.0 + b.1 * dim.1)));
        Self {
            blocks,
            dim,
            next: AtomicU32::new(0),
        }
    }
    pub fn next(&self) -> Option<(u32,u32)> {
        let i=self.next.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        if i>=self.blocks.len() as u32{
            None
        }else{
            Some(self.blocks[i as usize])
        }
    }
}
