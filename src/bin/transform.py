#%%
import pandas as pd
import numpy as np


#%%
data = pd.read_csv("../../data/standing_desk/standing_desk.csv")
# data = pd.read_csv("../../data/floating_wall_shelves.csv")

data.head()
#%%
# transform the catagorical data into numerical data
def transform_price_to_float(col_name):
    new_price = data[col_name].str.replace("$", "")
    data[col_name] = new_price.astype(float)


transform_price_to_float("Price")
transform_price_to_float("FBA Fees")

#%%
# Remove commas
def remove_commas(col_name):
    new_price = data[col_name].str.replace(",", "")
    data[col_name] = new_price.astype(float)


remove_commas("Revenue")
remove_commas("BSR")
remove_commas("Sales")
remove_commas("Review velocity")

# %%

# %%
data.to_csv("../../data/standing_desk/standing_desk_numerical.csv")

# %%
data
# %%
data.head()
# %%
