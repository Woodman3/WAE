import json
import pdb
# pdb.set_trace()

# 打开并读取JSON文件
with open('./skill_table.json', 'r',encoding='utf-8') as f:
    data = json.load(f)

# 创建一个空的列表，用于存储所有的`skill type`键的值
skill_types = []
duration = []
sp_types =[]

# 遍历解析后的Python对象，将每个对象的`skill type`键的值添加到列表中
for item in data.values():
    if 'skillType' in item['levels'][0]:
        skill = item["levels"][0]['skillType']
        if skill not in skill_types:
            skill_types.append(item["levels"][0]['skillType'])
            # 遍历解析后的Python对象，将每个对象的`duration type`键的值添加到列表中
    if 'durationType' in item['levels'][0]:
        duration_type = item["levels"][0]['durationType']
        if duration_type not in duration:
            duration.append(duration_type)
    if 'spType' in item['levels'][0]['spData']:
        sp_type = item["levels"][0]['spData']['spType']
        if sp_type not in sp_types:
            sp_types.append(sp_type) 

# 打印列表
print(duration)
# 打印列表
print(skill_types)
print(sp_types)