
use crate::extends::Point2;
use cgmath::EuclideanSpace;
use rand::Rng;

use super::CameraSample;
#[derive(Debug,Clone)]
pub struct StratifiedSampler {
    pub samples_per_pixel: i64,
    x_pixel_samples: i32,
    y_pixel_samples: i32,
    jitter_samples: bool,

    samples_1d: Vec<Vec<f64>>,
    samples_2d: Vec<Vec<Point2>>,
    current_1d_dimension: i32,
    current_2d_dimension: i32,

    rng: rand::ThreadRng,

    current_pixel: Point2,
    current_pixel_sample_index: i64,
    samples_1d_array_sizes: Vec<i32>,
    samples_2d_array_sizes: Vec<i32>,
    sample_array_1d: Vec<Vec<f64>>,
    sample_array_2d: Vec<Vec<Point2>>,
    array_1d_offset: usize,
    array_2d_offset: usize,
}

impl StratifiedSampler {
    pub fn new(
        x_pixel_samples: i32,
        y_pixel_samples: i32,
        jitter_samples: bool,
        n_sampled_dimensions: i64,
    ) -> Self {
        let mut ss = StratifiedSampler {
            samples_per_pixel: (x_pixel_samples * y_pixel_samples) as i64,
            x_pixel_samples,
            y_pixel_samples,
            jitter_samples,
            samples_1d: Vec::new(),
            samples_2d: Vec::new(),
            current_1d_dimension: 0_i32,
            current_2d_dimension: 0_i32,
            rng: rand::thread_rng(),
            current_pixel: Point2::origin(),
            current_pixel_sample_index: 0_i64,
            samples_1d_array_sizes: Vec::new(),
            samples_2d_array_sizes: Vec::new(),
            sample_array_1d: Vec::new(),
            sample_array_2d: Vec::new(),
            array_1d_offset: 0_usize,
            array_2d_offset: 0_usize,
        };
        for _i in 0..n_sampled_dimensions {
            let additional_1d: Vec<f64> = vec![0.0; ss.samples_per_pixel as usize];
            let additional_2d: Vec<Point2> = vec![Point2::origin(); ss.samples_per_pixel as usize];
            ss.samples_1d.push(additional_1d);
            ss.samples_2d.push(additional_2d);
        }
        ss
    }
    pub fn start_pixel(&mut self, p: Point2) {
        for i in 0..self.samples_1d.len() {
            stratified_sample_1d(
                self.samples_1d[i].as_mut_slice(),
                self.x_pixel_samples,
                &mut self.rng,
                self.jitter_samples,
            );
            self.rng.shuffle(self.samples_1d.as_mut_slice());
        }
        for i in 0..self.sample_array_2d.len() {
            stratified_sample_2d(
                self.samples_2d[i].as_mut_slice(),
                self.x_pixel_samples,
                self.y_pixel_samples,
                &mut self.rng,
                self.jitter_samples,
            );
            self.rng.shuffle(self.samples_2d[i].as_mut_slice());
        }
        for i in 0..self.samples_1d_array_sizes.len() {
            for j in 0..self.samples_per_pixel {
                let count: i32 = self.samples_1d_array_sizes[i as usize];
                let samples: &mut [f64] =
                    &mut self.sample_array_1d[i][(j as usize * count as usize)..];
                stratified_sample_1d(samples, count, &mut self.rng, self.jitter_samples);
                self.rng.shuffle(samples)
            }
        }
        for i in 0..self.samples_2d_array_sizes.len() {
            for j in 0..self.samples_per_pixel {
                let count: u32 = self.samples_2d_array_sizes[i as usize] as u32;
                latin_hypercube(
                    &mut self.sample_array_2d[i as usize][(j as usize * count as usize)..],
                    count,
                    &mut self.rng,
                );
            }
        }
        // PixelSampler::StartPixel(p);
        self.current_pixel = p;
        self.current_pixel_sample_index = 0_i64;
        // reset array offsets for next pixel sample
        self.array_1d_offset = 0_usize;
        self.array_2d_offset = 0_usize;
        self.current_pixel = p;
        self.array_1d_offset = 0;
        self.array_2d_offset = 0;
    }
    pub fn get_1d(&mut self) -> f64 {
        assert!(self.current_pixel_sample_index < self.samples_per_pixel);
        if self.current_1d_dimension < self.samples_1d.len() as i32 {
            let sample: f64 = self.samples_1d[self.current_1d_dimension as usize]
                [self.current_pixel_sample_index as usize];
            self.current_1d_dimension += 1;
            sample
        } else {
            self.rng.next_f64()
        }
    }
    pub fn get_2d(&mut self) -> Point2 {
        // TODO: ProfilePhase _(Prof::GetSample);
        assert!(self.current_pixel_sample_index < self.samples_per_pixel);
        if self.current_2d_dimension < self.samples_2d.len() as i32 {
            let sample: Point2 = self.samples_2d[self.current_2d_dimension as usize]
                [self.current_pixel_sample_index as usize];
            self.current_2d_dimension += 1;
            sample
        } else {
            // C++ call order for Point2(rng.UniformFloat(), rng.UniformFloat());
            let y = self.rng.next_f64();
            let x = self.rng.next_f64();
            Point2::new(x, y)
        }
    }
    pub fn get_2d_sample(&self, array_idx: usize, idx: usize) -> Point2 {
        self.sample_array_2d[array_idx][idx]
    }
    pub fn request_2d_array(&mut self, n: i32) {
        assert_eq!(self.round_count(n), n);
        self.samples_2d_array_sizes.push(n);
        let size: usize = (n * self.samples_per_pixel as i32) as usize;
        let additional_points: Vec<Point2> = vec![Point2::origin(); size];
        self.sample_array_2d.push(additional_points);
    }
    pub fn round_count(&self, count: i32) -> i32 {
        count
    }
    pub fn get_2d_array(&mut self, n: i32) -> Option<&[Point2]> {
        if self.array_2d_offset == self.sample_array_2d.len() {
            return None;
        }
        assert_eq!(self.samples_2d_array_sizes[self.array_2d_offset], n);
        assert!(self.current_pixel_sample_index < self.samples_per_pixel);
        let start: usize = (self.current_pixel_sample_index * n as i64) as usize;
        let end: usize = start + n as usize;
        self.array_2d_offset += 1;
        Some(&self.sample_array_2d[self.array_2d_offset - 1][start..end])
    }
    pub fn get_2d_array_idxs(&mut self, n: i32) -> (bool, usize, usize) {
        if self.array_2d_offset == self.sample_array_2d.len() {
            return (true, 0_usize, 0_usize);
        }
        assert_eq!(self.samples_2d_array_sizes[self.array_2d_offset], n);
        assert!(self.current_pixel_sample_index < self.samples_per_pixel);
        let start: usize = (self.current_pixel_sample_index * n as i64) as usize;
        let idx: usize = self.array_2d_offset;
        self.array_2d_offset += 1;
        (false, idx, start)
    }
    pub fn start_next_sample(&mut self) -> bool {
        self.current_1d_dimension = 0_i32;
        self.current_2d_dimension = 0_i32;
        // Sampler::StartNextSample()
        // reset array offsets for next pixel sample
        self.array_1d_offset = 0_usize;
        self.array_2d_offset = 0_usize;
        self.current_pixel_sample_index += 1_i64;
        self.current_pixel_sample_index < self.samples_per_pixel
    }
    pub fn reseed(&mut self, _seed: u64) {
        self.rng=rand::thread_rng();
        unimplemented!()
    }
    pub fn get_current_pixel(&self) -> Point2 {
        self.current_pixel
    }
    pub fn get_current_sample_number(&self) -> i64 {
        self.current_pixel_sample_index
    }
    pub fn get_samples_per_pixel(&self) -> i64 {
        self.samples_per_pixel
    }
    pub fn get_camera_sample(&mut self,p: Point2)->CameraSample{
        let d=self.get_2d();
        let p=Point2::new(p.x+d.x,p.y+d.y);
        CameraSample::new_all(p, self.get_1d(),self.get_2d())
    }
}
//在[0,1]采样nSample个点,每个点相对中心随机偏移
fn stratified_sample_1d(
    sampe: &mut [f64],
    n_samples: i32,
    rng: &mut rand::ThreadRng,
    jitter: bool,
) {
    let inv_n_sample = 1.0 / n_samples as f64;
    let mut delta;
    for item in 0..n_samples {
        delta = if jitter { rng.next_f64() } else { 0.5 };
        sampe[item as usize] = f64::min((item as f64 + delta) * inv_n_sample, f64::MAX);
    }
}
fn latin_hypercube(samples: &mut [Point2], n_samples: u32, rng: &mut rand::ThreadRng) {
    let n_dim: usize = 2;
    // generate LHS samples along diagonal
    let inv_n_samples: f64 = 1.0 as f64 / n_samples as f64;
    for i in 0..n_samples {
        for j in 0..n_dim {
            let sj = (i as f64 + (rng.next_f64())) * inv_n_samples;
            if j == 0 {
                samples[i as usize].x = sj.min(f64::MAX);
            } else {
                samples[i as usize].y = sj.min(f64::MAX);
            }
        }
    }
    rng.shuffle(samples);
}
fn stratified_sample_2d(
    sampe: &mut [Point2],
    n_x: i32,
    n_y: i32,
    rng: &mut rand::ThreadRng,
    jitter: bool,
) {
    let dx = 1.0 / n_x as f64;
    let dy = 1.0 / n_y as f64;
    let mut delta_x ;
    let mut delta_y ;
    for i in 0..n_x as usize {
        for j in 0..n_y as usize {
            delta_x = if jitter { rng.next_f64() } else { 0.5 };
            delta_y = if jitter { rng.next_f64() } else { 0.5 };
            sampe[i * n_x as usize + j].x = ((i as f64 + delta_x) * dx).min(f64::MAX);
            sampe[i * n_x as usize + j].y = ((j as f64 + delta_y) * dy).min(f64::MAX);
        }
    }
}