import pandas as pd
from sklearn.decomposition import PCA
from sklearn.preprocessing import StandardScaler
from sklearn.datasets import load_iris
import matplotlib.pyplot as plt
import mpl_toolkits.mplot3d.axes3d as p3

iris_data = load_iris()
x = iris_data.data
y = iris_data.target

fig = plt.figure(figsize=(8, 6))
ax = p3.Axes3D(fig, elev=-150, azim=110)
ax.set_xlabel('Sepal length')
ax.set_ylabel('Sepal width')
ax.set_zlabel('Petal length')

for name, label in [('Setosa', 0), ('Versicolour', 1), ('Virginica', 2)]:
    ax.text3D(x[y == label, 0].mean(),
              x[y == label, 1].mean() + 1.5,
              x[y == label, 2].mean(), name,
              horizontalalignment='center',
              bbox=dict(alpha=.5, edgecolor='w', facecolor='w'))

# ax.scatter(x[:, 0], x[:, 1], x[:, 2], c=y, cmap='rainbow', edgecolor='k')

# plt.show()

# PCA starts here
pca = PCA(n_components=2)
x_pca = pca.fit_transform(x)

import numpy as np
np.choose(y, [1, 2, 0]).astype(np.float)
ax.scatter(x[:, 0], x[:, 1], x[:, 2], c=y, cmap='rainbow', edgecolor='k')
ax.w_xaxis.set_ticklabels([])
ax.w_yaxis.set_ticklabels([])
ax.w_zaxis.set_ticklabels([])

plt.show()
