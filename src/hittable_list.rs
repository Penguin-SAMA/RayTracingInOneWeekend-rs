use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(&self, object: Box<dyn Hittable>) -> Self {
        let mut list = Self::default();
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut best: Option<HitRecord> = None;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                best = Some(rec);
            }
        }

        best
    }
}
