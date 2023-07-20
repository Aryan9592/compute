use utils::arithmetic;
use crate::constants;
use ruint::Uint;

pub struct StrideCounter<'a> {
    interval: &'a Interval,
    total: u32,
    value: i32,
}

impl<'a> StrideCounter<'a> {
    pub fn new(interval: &Interval, total: u32, v: Option<i32>) -> StrideCounter {
        let value = match v {
            Some(value) => value,
            None => -1
        };
        StrideCounter {
            interval,
            total,
            value,
        }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn cycle(&self) -> u32 {
        let cycle = (self.value << (self.interval.log2_stride - constants::A)) as u32;
        cycle
    }

    fn ucycle(&self) -> u32 {
        let ucycle = (self.value << self.interval.log2_stride) as u32;
        ucycle
    }

    fn remaining_strides(&self) -> u32 {
        self.total - self.value as u32
    }
}

pub struct Interval {
    base_meta_counter: Uint<256, 4>,
    pub log2_stride: u32,
    log2_stride_count: u32,
}

impl Interval {
    pub fn new(base_meta_counter: Uint<256, 4>, log2_stride: u32, log2_stride_count: u32) -> Interval {
        Interval {
            base_meta_counter,
            log2_stride,
            log2_stride_count,
        }
    }

    fn _build_iter(&self, log2_total_strides: u32) -> (StrideCounter, u64, StrideCounter) {
        let total_strides = arithmetic::max_int(log2_total_strides);
        let stride = StrideCounter::new(self, total_strides.try_into().unwrap(), None);
        (istrides, total_strides, stride)
    }

    pub fn strides(&self) -> (StrideCounter, u64, StrideCounter) {
        self._build_iter(self.log2_stride_count)
    }

    pub fn big_strides(&self) -> (StrideCounter, u64, StrideCounter) {
        let bid_strides_in_interval = if self.log2_stride_count >= constants::A {
            self.log2_stride_count - constants::A
        } else {
            0
        };

        self._build_iter(bid_strides_in_interval)
    }

    pub fn total_ucycles_in_cycle(&self) -> u32 {
        let ucycles = std::cmp::min(constants::A, self.log2_stride_count);
        arithmetic::max_int(ucycles) as u32
    }

    pub fn ucycles_in_cycle(&self) -> (StrideCounter, u32, StrideCounter) {
        let total_strides = self.total_ucycles_in_cycle();
        let stride = StrideCounter::new(self, total_strides, None);
        (iustrides, total_strides, stride)
    }
}