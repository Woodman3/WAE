import gymnasium as gym
from gym.spaces import Discrete, Box, Dict
import numpy as numpy

import ctypes

class WAE(gym.Env):
    metadata = {'render_modes': []}

    def __init__(self,render_modes=None,config:str="./config"):
        self.dll = ctypes.dll("./target/debug/wae.dll")
        assert(dll.init(config)==0)
        self.buffer=ctypes.create_string_buffer(b'\0',100)
        self.observation_space =Dict({
            "map":Box(low=0,high=255,shape=(10,10)),
        })
        self.action_space = gym.spaces.Discrete(4)
        
    def step(self,action):



