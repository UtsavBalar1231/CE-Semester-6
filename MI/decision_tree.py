from IPython.display import display
import pandas as pd
# from sklearn.datasets import load_diabetes
from sklearn.model_selection import train_test_split
from sklearn import metrics, preprocessing
from sklearn.tree import DecisionTreeClassifier
from sklearn.metrics import accuracy_score
import matplotlib.pyplot as plt
import numpy as np

# diabetes_data = load_diabetes()

# x = diabetes_data.data
# y = diabetes_data.target

# print(df.head())
# print(df.describe())

# x_train, x_test, y_train, y_test = train_test_split(
#     x, y, test_size=0.2, random_state=42)

# dt = DecisionTreeClassifier()
# dt.fit(x_train, y_train)

# y_pred = dt.predict(x_test)

# from sklearn.metrics import accuracy_score
# print(accuracy_score(y_test, y_pred))

data = pd.read_csv('diabetes.csv')
# display(data.info(), data.head())

# Check for missing values
# print("nulldata", data.isnull().sum())

# Data Preprocessing
#preprocessing = False
preprocessing = True
if preprocessing:
    data['Glucose'].fillna(data['Glucose'].mean(), inplace=True)
    data['BloodPressure'].fillna(data['BloodPressure'].mean(), inplace=True)
    data['SkinThickness'].fillna(data['SkinThickness'].mean(), inplace=True)
    data['Insulin'].fillna(data['Insulin'].mean(), inplace=True)
    data['BMI'].fillna(data['BMI'].mean(), inplace=True)


# Correlation Matrix for Diabetes Data
corr = data.corr()
# print(corr.values)

# plt.figure(figsize=(17,7))
# import seaborn as sns
# sns.heatmap(corr, annot=True, cmap='RdYlGn')
# plt.show()


X = data.drop(['Outcome'], axis=1).values
y = data['Outcome'].values

# Split the data into training and testing sets
X_train, X_test, y_train, y_test = train_test_split(
    X, y, test_size=0.2, random_state=0)
print('training data : ', X_train.shape, y_train.shape)
print('testing data : ', X_test.shape, y_test.shape)

# Decision Tree Classifier
dtc = DecisionTreeClassifier()
dtc = dtc.fit(X_train, y_train)

y_pred = dtc.predict(X_test)
acc = accuracy_score(y_pred, y_test)

print('Accuracy is: ', acc)
