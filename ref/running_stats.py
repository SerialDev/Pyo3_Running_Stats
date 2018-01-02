import numpy as np
import math


class RunningStats:
    """
    O(1) space complexity Running Statistics (mean, variance, stdev) based on 
    1962 paper by B. P. Welford and is presented
    in Donald Knuth’s Art of Computer Programming, Vol 2, page 232, 
    """
    
    def __init__(self):
        self.n = 0
        self.old_m = 0
        self.new_m = 0
        self.old_s = 0
        self.new_s = 0
        self.min_ = float('+inf')
        self.max_ = float('-inf')

    def clear(self):
        self.n = 0

    def push(self, x, *args):
        self.n += 1
        self.min_ = min(self.min_, x)
        self.max_ = max(self.max_, x)

        if self.n == 1:
            self.old_m = self.new_m = x
            self.old_s = 0
        else:
            self.new_m = self.old_m + (x - self.old_m) / self.n
            self.new_s = self.old_s + (x - self.old_m) * (x - self.new_m)

            self.old_m = self.new_m
            self.old_s = self.new_s

    def mean(self):
        return self.new_m if self.n else 0.0

    def variance(self):
        return self.new_s / (self.n - 1) if self.n > 1 else 0.0

    def standard_deviation(self):
        return math.sqrt(self.variance())
    def max_num(self):
        return self.max_
    def min_num(self):
        return self.min_
