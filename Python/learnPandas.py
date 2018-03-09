# Pandas and Numpy
import pandas as pd
import numpy as np

# The key data structures in pandas are
# - Series
# - DataFrames
# - Panels (collection of DataFrames)

# Example Series and Dataframes
s = pd.Series([1, 3, 5, np.nan, 6, 8])
dates = pd.date_range('1/1/2000', periods=8,)
df = pd.DataFrame(np.random.randn(8, 4), index=dates,
                  columns=['A', 'B', 'C', 'D'])

# Print Those out
print("series: ", s)
print("dates: ", dates)
print("Dataframe: ", df)
