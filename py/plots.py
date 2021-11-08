#%%
import numpy as np
import pandas as pd
import plotly.express as px

#%%

dataset = "../listing_data/standing_desk/standing_desk.csv"

#%%
def load_data(path):
    data = pd.read_csv(path)

    def transform_price_to_float(col_name):
        new_price = data[col_name].str.replace("$", "")
        data[col_name] = new_price.astype(float)

    transform_price_to_float("Price")
    transform_price_to_float("FBA Fees")

    def remove_commas(col_name):
        new_price = data[col_name].str.replace(",", "")
        data[col_name] = new_price.astype(float)

    remove_commas("Revenue")
    remove_commas("BSR")
    remove_commas("Sales")
    remove_commas("Review velocity")

    return data


#%%
def plot_review_count_vs_velocity():
    data = load_data(dataset)
    fig = px.scatter(x=[0, 1, 2, 3, 4], y=[0, 1, 4, 9, 16])
    fig.show()


plot_review_velocity()

# %%
data = load_data(dataset)
# %%
data.head()
# %%
