import gymnasium as gym
import numpy as numpy

import ctypes
from ctypes import POINTER, c_char, c_char_p 

import json

class WAE(gym.Env):
    metadata = {'render_modes': []}

    def __init__(self,render_modes=None,config_path:str="./config"):
        self.lib_init("./libwae.so",config_path) 
        self.render_modes = render_modes
        self.get_frame()

        self.observation_space = gym.spaces.Text(10000)
        self.action_space = gym.spaces.Text(10000)

    def _get_obs(self):
        return self.frame

    def _get_info(self):
        return self.frame

    def step(self, action):
        self.lib.step(action)
        self.get_frame()
        return self.frame,0,False,{}

    
        
    def lib_init(lib_path:str,config_path:str):
        self.lib = ctypes.dll(lib_path)
        assert(lib.init(config_path)==0)
        self.lib.get_frame.argtypes = [POINTER(c_char)] 
        self.lib.step.argtypes = [c_char_p]


    def lib_get_str(self,func_name)->Optional[Dict]:
        try:
            func = getattr(self.lib,func_name)
            if func.restype != POINTER(c_char):
                return None
            ptr = func()
            data = ctypes.cast(ptr, c_char_p).value
            self.lib.free_str(ptr)
            return json.loads(data)
        except:
            f"Error: {func_name} not found in lib"
            return None
    
    def get_frame(self):
        self.frame = self.lib_get_str("get_frame")        

    


