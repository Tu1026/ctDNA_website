import joblib
import m2cgen as m2c
import sys

sys.setrecursionlimit(2147483647)
model = joblib.load('front_facing_xgb_with_cf.pkl')